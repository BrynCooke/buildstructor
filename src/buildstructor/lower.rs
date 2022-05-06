use crate::analyze::ConstrutorModel;
use crate::buildstructor::utils::{IdentExt, PunctuatedExt, TypeExt};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::default::Default;
use syn::punctuated::Punctuated;
use syn::{
    Expr, ExprField, FnArg, GenericArgument, GenericParam, Generics, Index, Member, Pat,
    PathArguments, Result, ReturnType, Type, TypeParam, TypeTuple, VisRestricted, Visibility,
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
    pub builder_vis: Visibility,
    pub generics: Generics,
    pub builder_generics: Generics,
    pub method_generics: Generics,
    pub builder_method_name: Ident,
}

pub struct BuilderField {
    pub name: Ident,
    pub ty: Type,
    pub into: bool,
    pub field_type: FieldType,
    pub key_type: Option<Type>,
    pub key_into: bool,
    pub value_type: Option<Type>,
    pub value_into: bool,
    pub generic_type: Option<Type>,
    pub generic_into: bool,
}

#[derive(Debug)]
pub enum FieldType {
    Regular,
    Option,
    Vec,
    Set,
    Map,
}

pub fn lower(model: ConstrutorModel) -> Result<Ir> {
    // Either visibility is set explicitly or we default to super.
    let vis = model.vis.clone();
    let builder_vis = builder_vilibility(&vis);

    Ok(Ir {
        vis,
        builder_vis,
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
        return_type: builder_return_type(model.output, &model.ident),
        is_async: model.is_async,
        generics: model.generics,
        builder_generics: Ir::builder_generics(),
        method_generics: model.method_generics,
    })
}

fn builder_vilibility(vis: &Visibility) -> Visibility {
    if let Visibility::Inherited = vis {
        Visibility::Restricted(VisRestricted {
            pub_token: Default::default(),
            paren_token: Default::default(),
            in_token: None,
            path: Box::new(format_ident!("super").to_path()),
        })
    } else {
        vis.clone()
    }
}

fn builder_return_type(mut return_type: ReturnType, target: &Ident) -> ReturnType {
    if let ReturnType::Type(_, ty) = &mut return_type {
        replace_self(ty, target);
    }
    return_type
}

fn replace_self(ty: &mut Type, target: &Ident) {
    let self_type = format_ident!("Self").to_type_path();
    if let Type::Path(path) = ty {
        if path == &self_type {
            *path = target.to_type_path();
        } else {
            for segment in path.path.segments.iter_mut() {
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for mut arg in args.args.iter_mut() {
                        if let GenericArgument::Type(ty) = &mut arg {
                            replace_self(ty, target);
                        }
                    }
                }
            }
        }
    }
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
                let ((key_type, key_into), (value_type, value_into), (generic_type, generic_into)) =
                    match (
                        &field_type,
                        args.and_then(|args| args.iter().next()),
                        args.and_then(|args| args.iter().nth(1)),
                    ) {
                        (
                            FieldType::Option | FieldType::Vec | FieldType::Set,
                            Some(GenericArgument::Type(collection_type)),
                            None,
                        ) => (
                            (None, false),
                            (None, false),
                            (
                                Some(collection_type.clone()),
                                collection_type
                                    .is_into_capable(&model.generics, &model.method_generics),
                            ),
                        ),
                        (
                            FieldType::Map,
                            Some(GenericArgument::Type(key_type)),
                            Some(GenericArgument::Type(value_type)),
                        ) => (
                            (
                                Some(key_type.clone()),
                                key_type.is_into_capable(&model.generics, &model.method_generics),
                            ),
                            (
                                Some(value_type.clone()),
                                value_type.is_into_capable(&model.generics, &model.method_generics),
                            ),
                            (None, false),
                        ),
                        _ => ((None, false), (None, false), (None, false)),
                    };

                let into =
                    t.ty.is_into_capable(&model.generics, &model.method_generics);
                Some(BuilderField {
                    ty: *t.ty.clone(),
                    into,
                    name: ident.ident.clone(),
                    field_type,
                    key_type,
                    key_into,
                    value_type,
                    value_into,
                    generic_type,
                    generic_into,
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
        Some(f) if f == format_ident!("Option") => FieldType::Option,
        Some(f) if f == format_ident!("Vec") => FieldType::Vec,
        Some(f) if f.to_string().ends_with("Stack") => FieldType::Vec,
        Some(f) if f.to_string().ends_with("Heap") => FieldType::Vec,
        Some(f) if f.to_string().ends_with("Deque") => FieldType::Vec,
        Some(f) if f.to_string().ends_with("Buffer") => FieldType::Vec,
        Some(f) if f.to_string().ends_with("Set") => FieldType::Set,
        Some(f) if f.to_string().ends_with("Map") => FieldType::Map,
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
