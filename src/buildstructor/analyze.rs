use crate::buildstructor::utils::TypeExt;
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
}

#[derive(Default)]
pub struct BuilderConfig {
    pub entry: Option<String>,
    pub exit: Option<String>,
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

        Ok(match value.parse_meta()? {
            Meta::Path(_) => BuilderConfig::default(),
            Meta::List(l) => {
                let mut config = BuilderConfig::default();
                for nested in l.nested {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                        apply(&mut config, &name_value)?;
                    }
                }
                config
            }
            Meta::NameValue(name_value) => {
                let mut config = BuilderConfig::default();
                apply(&mut config, &name_value)?;
                config
            }
        })
    }
}

pub fn analyze(ast: &Ast) -> Result<Vec<Result<BuilderModel>>> {
    let constructors = get_constructors(&ast.item);
    let ident = ast
        .item
        .self_ty
        .raw_ident()
        .ok_or_else(|| syn::Error::new(ast.item.span(), "cannot find name of struct"))?;
    let constructor_models = constructors
        .into_iter()
        .map(|(builder, attr)| {
            Ok(BuilderModel {
                impl_name: ident.clone(),
                impl_generics: ast.item.generics.clone(),
                delegate_name: builder.sig.ident.clone(),
                delegate_generics: builder.sig.generics.clone(),
                delegate_args: builder.sig.inputs.clone().into_iter().collect(),
                delegate_return_type: builder.sig.output.clone(),
                is_async: builder.sig.asyncness.is_some(),
                vis: builder.vis.clone(),
                config: attr.try_into()?,
            })
        })
        .collect();

    Ok(constructor_models)
}

fn get_constructors(item: &ItemImpl) -> Vec<(&ImplItemMethod, &Attribute)> {
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
                    return Some((method, attr));
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
        analyze(&single_field_test_case()).unwrap();
    }

    #[test]
    fn pub_test() {
        analyze(&pub_test_case()).unwrap();
    }

    #[test]
    fn multi_field_test() {
        analyze(&multi_field_test_case()).unwrap();
    }

    #[test]
    fn generic_test() {
        analyze(&generic_test_case()).unwrap();
    }

    #[test]
    fn async_test() {
        analyze(&async_test_case()).unwrap();
    }

    #[test]
    fn fallible_test() {
        analyze(&fallible_test_case()).unwrap();
    }

    #[test]
    fn into_test() {
        analyze(&into_test_case()).unwrap();
    }

    #[test]
    fn into_where_test() {
        analyze(&into_where_test_case()).unwrap();
    }

    #[test]
    fn option_test() {
        analyze(&option_test_case()).unwrap();
    }

    #[test]
    fn collection_test() {
        analyze(&collections_test_case()).unwrap();
    }

    #[test]
    fn collection_generics_test() {
        analyze(&collections_generics_test_case()).unwrap();
    }
}
