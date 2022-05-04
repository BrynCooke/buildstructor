#![doc = include_str!("../README.md")]
extern crate core;

use proc_macro::TokenStream;
use quote::ToTokens;

mod buildstructor;
use crate::buildstructor::analyze;
use crate::buildstructor::codegen;
use crate::buildstructor::lower;
use crate::buildstructor::parse;

/// Derive a builder from a constructor!
///
/// 1. Import the `builder` macro.
/// 2. Annotate your `impl` containing a `new` function.
/// 3. Use your automatically derived builder.
///
/// TLDR: Write your Rust constructors as you would normally, and get a generated builder.
///
/// # Examples
///
/// ```rust
/// use buildstructor::builder;
///
/// struct MyStruct {
///     sum: usize,
/// }
///
/// #[builder]
/// impl MyStruct {
///     fn new(a: usize, b: usize) -> MyStruct {
///         Self { sum: a + b }
///     }
/// }
/// # #[allow(clippy::needless_doctest_main)]
/// # fn main() {
///   let mine = MyStruct::builder().a(2).b(3).build();
///   assert_eq!(mine.sum, 5);
/// # }
/// ```
#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    match process(attr, item.clone()) {
        Ok(ok) => ok,
        Err(err) => TokenStream::from_iter([item, err]),
    }
}

fn process(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
    let ast = parse::parse(TokenStream::from_iter(vec![attr, item]).into())
        .map_err(|e| e.into_compile_error())?;
    let constructors = analyze::analyze(ast.clone()).map_err(|e| e.into_compile_error())?;
    let mut token_streams: Vec<proc_macro::TokenStream> = Vec::new();
    let original_token_stream = ast.item.to_token_stream().into();
    token_streams.push(original_token_stream);

    for constructor_model in constructors {
        let ir = lower::lower(constructor_model).map_err(|e| e.into_compile_error())?;

        let code_gen = codegen::codegen(ir).map_err(|e| e.into_compile_error())?;
        token_streams.push(code_gen.into());
    }

    Ok(TokenStream::from_iter(token_streams.into_iter()))
}
