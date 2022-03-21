use crate::buildstructor::utils::TypeExt;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{
    FnArg, Generics, Ident, ImplItem, ImplItemMethod, ItemImpl, Result, ReturnType, Visibility,
};

use crate::parse::Ast;
pub struct Model {
    pub ident: Ident,
    pub generics: Generics,
    pub method_generics: Generics,
    pub args: Vec<FnArg>,
    pub output: ReturnType,
    pub is_async: bool,
    pub vis: Visibility,
}

pub fn analyze(ast: Ast) -> Result<Model> {
    let constructor = get_constructor(&ast.item).ok_or_else(|| {
        syn::Error::new(
            ast.item.span(),
            "Cannot find 'new' function with no receiver.",
        )
    })?;
    let result = Model {
        ident: ast
            .item
            .self_ty
            .raw_ident()
            .ok_or_else(|| syn::Error::new(ast.item.span(), "Cannot find name of struct."))?,
        generics: ast.item.generics.clone(),
        method_generics: constructor.sig.generics.clone(),
        args: constructor.sig.inputs.clone().into_iter().collect(),
        output: constructor.sig.output.clone(),
        is_async: constructor.sig.asyncness.is_some(),
        vis: constructor.vis.clone(),
    };

    Ok(result)
}

fn get_constructor(item: &ItemImpl) -> Option<&ImplItemMethod> {
    for item in &item.items {
        if let ImplItem::Method(f) = item {
            if f.sig.ident == format_ident!("new")
                && !f.sig.inputs.iter().any(|a| matches!(a, FnArg::Receiver(_)))
            {
                return Some(f);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buildstructor::tests::*;

    #[test]
    fn single_field_test() {
        analyze(single_field_test_case()).unwrap();
    }

    #[test]
    fn pub_test() {
        analyze(pub_test_case()).unwrap();
    }

    #[test]
    fn multi_field_test() {
        analyze(multi_field_test_case()).unwrap();
    }

    #[test]
    fn generic_test() {
        analyze(generic_test_case()).unwrap();
    }

    #[test]
    fn async_test() {
        analyze(async_test_case()).unwrap();
    }

    #[test]
    fn fallible_test() {
        analyze(fallible_test_case()).unwrap();
    }

    #[test]
    fn into_test() {
        analyze(into_test_case()).unwrap();
    }

    #[test]
    fn into_where_test() {
        analyze(into_where_test_case()).unwrap();
    }

    #[test]
    fn option_test() {
        analyze(option_test_case()).unwrap();
    }

    #[test]
    fn collection_test() {
        analyze(collections_test_case()).unwrap();
    }
}
