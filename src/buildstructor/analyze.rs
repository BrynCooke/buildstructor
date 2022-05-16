use crate::buildstructor::utils::TypeExt;
use proc_macro2::Span;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{
    Attribute, FnArg, Generics, Ident, ImplItem, ImplItemMethod, ItemImpl, Lit, Meta,
    MetaNameValue, NestedMeta, Result, ReturnType, Visibility,
};

use crate::parse::Ast;
pub struct BuilderModel {
    pub impl_name: Ident,
    pub impl_generics: Generics,
    pub delegate_name: Ident,
    pub delegate_generics: Generics,
    pub delegate_args: Vec<FnArg>,
    pub delegate_return_type: ReturnType,
    pub is_async: bool,
    pub vis: Visibility,
    pub config: BuilderConfig,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Default)]
pub struct BuildstructorConfig {}

impl TryFrom<&Vec<Attribute>> for BuildstructorConfig {
    type Error = syn::Error;

    fn try_from(attributes: &Vec<Attribute>) -> std::result::Result<Self, Self::Error> {
        fn apply(_config: &mut BuildstructorConfig, name_value: &MetaNameValue) -> Result<()> {
            let name = name_value
                .path
                .get_ident()
                .expect("config ident must be preset, qed");
            return Err(syn::Error::new(
                name_value.span(),
                format!("invalid buildstructor attribute '{}'", name),
            ));
        }

        let mut config = BuildstructorConfig::default();
        for attribute in attributes {
            match attribute.parse_meta()? {
                Meta::List(l) => {
                    for nested in l.nested {
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                            apply(&mut config, &name_value)?;
                        }
                    }
                }
                Meta::NameValue(name_value) => {
                    let mut config = BuildstructorConfig::default();
                    apply(&mut config, &name_value)?;
                }
                _ => {}
            }
        }
        Ok(config)
    }
}

#[derive(Default)]
pub struct BuilderConfig {
    pub entry: Option<String>,
    pub exit: Option<String>,
    pub span: Option<Span>,
}

impl TryFrom<&Attribute> for BuilderConfig {
    type Error = syn::Error;

    fn try_from(value: &Attribute) -> std::result::Result<Self, Self::Error> {
        fn apply(config: &mut BuilderConfig, name_value: &MetaNameValue) -> Result<()> {
            let name = name_value
                .path
                .get_ident()
                .expect("config ident must be preset, qed");
            let value = &name_value.lit;
            match (name.to_string().as_str(), value) {
                ("entry", Lit::Str(value)) => {
                    config.entry = Some(value.value());
                }
                ("exit", Lit::Str(value)) => {
                    config.exit = Some(value.value());
                }
                _ => return Err(syn::Error::new(
                    value.span(),
                    format!("invalid builder attribute '{}', only 'entry' and 'exit' are allowed and their type must be string", name),
                )),
            }
            Ok(())
        }

        let mut config = BuilderConfig {
            span: Some(value.span()),
            ..Default::default()
        };
        match value.parse_meta()? {
            Meta::Path(_) => {}
            Meta::List(l) => {
                for nested in l.nested {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                        apply(&mut config, &name_value)?;
                    }
                }
            }
            Meta::NameValue(name_value) => {
                apply(&mut config, &name_value)?;
            }
        }
        Ok(config)
    }
}

pub fn analyze(legacy_default_builders: bool, ast: &Ast) -> Result<Vec<Result<BuilderModel>>> {
    let _buildstructor_config: BuildstructorConfig = (&ast.attributes).try_into()?;
    let methods = get_eligible_methods(&ast.item, legacy_default_builders);
    let ident = ast
        .item
        .self_ty
        .raw_ident()
        .ok_or_else(|| syn::Error::new(ast.item.span(), "cannot find name of struct"))?;
    let models = methods
        .into_iter()
        .map(|(builder, config)| {
            Ok(BuilderModel {
                impl_name: ident.clone(),
                impl_generics: ast.item.generics.clone(),
                delegate_name: builder.sig.ident.clone(),
                delegate_generics: builder.sig.generics.clone(),
                delegate_args: builder.sig.inputs.clone().into_iter().collect(),
                delegate_return_type: builder.sig.output.clone(),
                is_async: builder.sig.asyncness.is_some(),
                vis: builder.vis.clone(),
                config: config?,
                attributes: builder.attrs.clone(),
            })
        })
        .collect();

    Ok(models)
}

fn get_eligible_methods(
    item: &ItemImpl,
    default_builders: bool,
) -> Vec<(&ImplItemMethod, Result<BuilderConfig>)> {
    item.items
        .iter()
        .filter_map(|item| {
            let builder_attr = Some(format_ident!("builder"));
            if let ImplItem::Method(method) = item {
                if let Some(attr) = method
                    .attrs
                    .iter()
                    .find(|attr| attr.path.get_ident() == builder_attr.as_ref())
                {
                    return Some((method, attr.try_into()));
                } else if default_builders {
                    // If the method doesn't have a receiver and it matches the new pattern.
                    let method_name = method.sig.ident.to_string();
                    if !matches!(method.sig.inputs.iter().next(), Some(FnArg::Receiver(_)))
                        && method_name.ends_with("_new")
                        || method_name.eq("new")
                    {
                        return Some((method, Ok(BuilderConfig::default())));
                    }
                }
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buildstructor::tests::*;

    #[test]
    fn single_field_test() {
        analyze(false, &single_field_test_case()).unwrap();
    }

    #[test]
    fn pub_test() {
        analyze(false, &pub_test_case()).unwrap();
    }

    #[test]
    fn multi_field_test() {
        analyze(false, &multi_field_test_case()).unwrap();
    }

    #[test]
    fn generic_test() {
        analyze(false, &generic_test_case()).unwrap();
    }

    #[test]
    fn async_test() {
        analyze(false, &async_test_case()).unwrap();
    }

    #[test]
    fn fallible_test() {
        analyze(false, &fallible_test_case()).unwrap();
    }

    #[test]
    fn into_test() {
        analyze(false, &into_test_case()).unwrap();
    }

    #[test]
    fn into_where_test() {
        analyze(false, &into_where_test_case()).unwrap();
    }

    #[test]
    fn option_test() {
        analyze(false, &option_test_case()).unwrap();
    }

    #[test]
    fn collection_test() {
        analyze(false, &collections_test_case()).unwrap();
    }

    #[test]
    fn collection_generics_test() {
        analyze(false, &collections_generics_test_case()).unwrap();
    }
}
