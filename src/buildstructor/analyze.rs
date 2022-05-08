use crate::buildstructor::utils::TypeExt;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{
    FnArg, Generics, Ident, ImplItem, ImplItemMethod, ItemImpl, Result, ReturnType, Visibility,
};

use crate::parse::Ast;
pub struct ConstrutorModel {
    pub ident: Ident,
    pub constructor_name: Ident,
    pub generics: Generics,
    pub method_generics: Generics,
    pub args: Vec<FnArg>,
    pub output: ReturnType,
    pub is_async: bool,
    pub vis: Visibility,
}

pub fn analyze(ast: &Ast) -> Result<Vec<ConstrutorModel>> {
    let constructors = get_constructors(&ast.item);
    let ident = ast
        .item
        .self_ty
        .raw_ident()
        .ok_or_else(|| syn::Error::new(ast.item.span(), "cannot find name of struct"))?;
    let constructor_models = constructors
        .into_iter()
        .map(|constructor| ConstrutorModel {
            ident: ident.clone(),
            constructor_name: constructor.sig.ident.clone(),
            generics: ast.item.generics.clone(),
            method_generics: constructor.sig.generics.clone(),
            args: constructor.sig.inputs.clone().into_iter().collect(),
            output: constructor.sig.output.clone(),
            is_async: constructor.sig.asyncness.is_some(),
            vis: constructor.vis.clone(),
        })
        .collect();

    Ok(constructor_models)
}

fn get_constructors(item: &ItemImpl) -> Vec<&ImplItemMethod> {
    item.items
        .iter()
        .filter_map(|item| {
            let builder_attr = Some(format_ident!("builder"));
            if let ImplItem::Method(m) = item {
                if m.attrs
                    .iter()
                    .any(|attr| attr.path.get_ident() == builder_attr.as_ref())
                {
                    return Some(m);
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
