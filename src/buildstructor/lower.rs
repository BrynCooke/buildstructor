use crate::analyze::ConstrutorModel;
use crate::buildstructor::utils::{IdentExt, PunctuatedExt, TypeExt};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::default::Default;
use syn::punctuated::Punctuated;
use syn::{
    Expr, ExprField, FnArg, GenericArgument, GenericParam, Generics, Index, Member, Pat, Result,
    ReturnType, Type, TypeParam, TypeTuple, Visibility,
};
use try_match::try_match;

pub struct Ir {
    pub module_name: Ident,
    pub target_name: Ident,
    pub builder_name: Ident,
    pub builder_fields: Vec<BuilderField>,
    pub constructor_name: Ident,
    pub constructor_method_name: Ident,
    pub return_type: ReturnType,
    pub is_async: bool,
    pub vis: Visibility,
    pub generics: Generics,
    pub builder_generics: Generics,
    pub method_generics: Generics,
    pub builder_method_name: Ident,
}

pub struct BuilderField {
    pub name: Ident,
    pub ty: Type,
    pub field_type: FieldType,
    pub key_type: Option<Type>,
    pub value_type: Option<Type>,
    pub collection_type: Option<Type>,
}

#[derive(Debug)]
pub enum FieldType {
    Regular,
    Optional,
    Vec,
    Set,
    Map,
}

pub fn lower(model: ConstrutorModel) -> Result<Ir> {
    Ok(Ir {
        vis: model.vis.clone(),
        module_name: format_ident!(
            "__{}_{}_builder",
            model.ident.to_string().to_lowercase(),
            model.constructor_name.to_string().to_lowercase()
        ),
        target_name: model.ident.clone(),
        builder_name: format_ident!("__{}Builder", model.ident.clone()),
        constructor_method_name: model.constructor_name.clone(),
        builder_method_name: builder_method_name(&model),
        builder_fields: builder_fields(&model),
        constructor_name: format_ident!("{}Constructor", model.ident.to_string()),
        return_type: builder_return_type(model.output, model.ident),
        is_async: model.is_async,
        generics: model.generics,
        builder_generics: Ir::builder_generics(),
        method_generics: model.method_generics,
    })
}

fn builder_return_type(mut return_type: ReturnType, target: Ident) -> ReturnType {
    if let ReturnType::Type(_, ty) = &mut return_type {
        let self_type = Box::new(Type::Path(format_ident!("Self").to_type_path()));
        if ty == &self_type {
            *ty = Box::new(Type::Path(target.to_type_path()));
        }
    }
    return_type
}

fn builder_fields(model: &ConstrutorModel) -> Vec<BuilderField> {
    model
        .args
        .iter()
        .filter_map(|f| match f {
            FnArg::Typed(t) => {
                let ident = try_match!(&*t.pat, Pat::Ident(x)=>x).ok()?;
                let field_type = field_type(&*t.ty);
                let args = t.ty.generic_args();
                let (key_type, value_type, collection_type) = match (
                    &field_type,
                    args.and_then(|args| args.iter().next()),
                    args.and_then(|args| args.iter().nth(1)),
                ) {
                    (
                        FieldType::Vec | FieldType::Set,
                        Some(GenericArgument::Type(collection_type)),
                        None,
                    ) => (None, None, Some(collection_type.clone())),
                    (
                        FieldType::Map,
                        Some(GenericArgument::Type(key_type)),
                        Some(GenericArgument::Type(value_type)),
                    ) => (Some(key_type.clone()), Some(value_type.clone()), None),
                    _ => (None, None, None),
                };

                Some(BuilderField {
                    ty: *t.ty.clone(),
                    name: ident.ident.clone(),
                    field_type,
                    key_type,
                    value_type,
                    collection_type,
                })
            }
            FnArg::Receiver(_) => None,
        })
        .collect()
}

fn builder_method_name(model: &ConstrutorModel) -> Ident {
    format_ident!(
        "{}builder",
        model
            .constructor_name
            .to_string()
            .strip_suffix("new")
            .expect("already checked that the method ends with new, qed")
    )
}

fn field_type(ty: &Type) -> FieldType {
    match ty.raw_ident() {
        Some(f) if f == format_ident!("Option") => FieldType::Optional,
        Some(f) if f == format_ident!("Vec") => FieldType::Vec,
        Some(f) if f == format_ident!("HashSet") => FieldType::Set,
        Some(f) if f == format_ident!("BTreeSet") => FieldType::Set,
        Some(f) if f == format_ident!("HashMap") => FieldType::Map,
        Some(f) if f == format_ident!("BTreeMap") => FieldType::Map,
        _ => FieldType::Regular,
    }
}

impl Ir {
    pub fn constructor_args(&self) -> Vec<TokenStream> {
        self.builder_fields
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let idx = Index::from(idx);
                quote! {
                    self.fields.#idx.into().value
                }
            })
            .collect()
    }

    pub fn tuple_field(&self, idx: usize) -> Expr {
        Expr::Field(ExprField {
            attrs: vec![],
            base: Box::new(Self::fields()),
            dot_token: Default::default(),
            member: Member::Unnamed(Index::from(idx)),
        })
    }

    pub fn fields() -> Expr {
        Expr::Field(ExprField {
            attrs: vec![],
            base: Box::new(Expr::Path(format_ident!("self").to_expr_path())),
            dot_token: Default::default(),
            member: Member::Named(format_ident!("fields")),
        })
    }

    pub fn builder_state_type_initial(&self) -> Type {
        Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(self.builder_fields.iter().map(|field| {
                match field.field_type {
                    FieldType::Regular => field.ty.wrap_in_generic_with_module(
                        &self.module_name,
                        format_ident!("__Required"),
                    ),
                    _ => field.ty.wrap_in_generic_with_module(
                        &self.module_name,
                        format_ident!("__Optional"),
                    ),
                }
            }))
            .with_trailing(),
        })
    }

    pub fn builder_state_initial(&self) -> Vec<TokenStream> {
        self.builder_fields
            .iter()
            .map(|field| match field.field_type {
                FieldType::Regular => quote! {__required()},
                _ => quote! {__optional()},
            })
            .collect()
    }

    pub fn param_generics(&self) -> Generics {
        Generics {
            params: Punctuated::from_iter(self.builder_fields.iter().enumerate().map(
                |(idx, f)| {
                    format_ident!("__P{}", idx).to_generic_param(Some(
                        &f.ty
                            .wrap_in_generic(format_ident!("__Set"))
                            .wrap_in_generic(format_ident!("Into")),
                    ))
                },
            )),
            ..Default::default()
        }
    }

    fn builder_generics() -> Generics {
        Generics {
            params: Punctuated::from_iter(vec![format_ident!("__P").to_generic_param(None)]),
            ..Default::default()
        }
    }

    pub fn builder_type_generics(&self) -> Generics {
        Generics {
            params: Punctuated::from_iter(
                self.builder_fields.iter().enumerate().map(|(idx, _f)| {
                    GenericParam::Type(TypeParam::from(format_ident!("__{}", idx)))
                }),
            ),
            ..Default::default()
        }
    }
}
