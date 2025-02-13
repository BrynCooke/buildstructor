use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, ItemImpl, Result};

#[derive(Clone, Debug)]
pub struct Ast {
    pub item: ItemImpl,
}

impl Parse for Ast {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Ast {
            item: input.parse()?,
        })
    }
}

pub fn parse(item: TokenStream) -> Result<Ast> {
    parse2::<Ast>(item)
}

#[cfg(test)]
mod tests {
    use crate::buildstructor::tests::{multi_field_test_case, single_field_test_case};

    #[test]
    fn single_field_test_case_is_valid() {
        single_field_test_case();
    }

    #[test]
    fn multiple_field_test_case_is_valid() {
        multi_field_test_case();
    }
}
