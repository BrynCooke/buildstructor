extern crate core;

use proc_macro::TokenStream;
use quote::ToTokens;

mod buildstructor;
use crate::buildstructor::analyze;
use crate::buildstructor::codegen;
use crate::buildstructor::lower;
use crate::buildstructor::parse;

#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    match process(attr, item) {
        Ok(ok) => ok,
        Err(err) => err,
    }
}

fn process(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
    let ast = parse::parse(TokenStream::from_iter(vec![attr, item]).into())
        .map_err(|e| e.into_compile_error())?;
    let model = analyze::analyze(ast.clone()).map_err(|e| e.into_compile_error())?;
    let ir = lower::lower(model).map_err(|e| e.into_compile_error())?;

    let code_gen = codegen::codegen(ir).map_err(|e| e.into_compile_error())?;
    let token_stream: TokenStream = ast.item.to_token_stream().into();

    Ok(TokenStream::from_iter(vec![token_stream, code_gen.into()]))
}
