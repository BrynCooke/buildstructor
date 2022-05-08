#![doc = include_str!("../README.md")]
extern crate core;

use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use syn::ImplItem;
use syn::__private::TokenStream2;

mod buildstructor;
use crate::buildstructor::analyze;
use crate::buildstructor::codegen;
use crate::buildstructor::lower;
use crate::buildstructor::parse;
use crate::parse::Ast;

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
/// use buildstructor::buildstructor;
///
/// struct MyStruct {
///     sum: usize,
/// }
///
/// #[buildstructor]
/// impl MyStruct {
///     #[builder]
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
pub fn buildstructor(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse::parse(TokenStream::from_iter(vec![attr, item.clone()]).into())
        .map_err(|e| e.into_compile_error())
    {
        Ok(mut ast) => {
            // We have the AST, we can return the token stream regardless of if there was success or not as long as we sanitize it of helper attributes.
            let mut results: Vec<proc_macro::TokenStream> = match analyze::analyze(&ast)
                .map_err(|e| e.into_compile_error())
            {
                Ok(builders) => builders
                    .into_iter()
                    .map(|builder| match builder {
                        Ok(builder) => {
                            let ir = lower::lower(builder).map_err(|e| e.into_compile_error())?;
                            let code_gen =
                                codegen::codegen(ir).map_err(|e| e.into_compile_error())?;
                            Ok(code_gen)
                        }
                        Err(e) => Err(e.into_compile_error()),
                    })
                    .map(|r: Result<TokenStream2, TokenStream2>| match r {
                        Ok(r) => r.into(),
                        Err(e) => e.into(),
                    })
                    .collect(),
                Err(e) => {
                    vec![e.into()]
                }
            };

            // Now sanitize the AST of any helper attributes.
            sanitize(&mut ast);

            // Finally output the results.
            let sanitized_token_stream = ast.item.to_token_stream();
            results.insert(0, sanitized_token_stream.into());
            TokenStream::from_iter(results)
        }
        Err(e) => {
            // The parse failed so emit the original token stream as some editors rely on this.
            TokenStream::from_iter([item, e.into()])
        }
    }
}

fn sanitize(ast: &mut Ast) {
    ast.item.items.iter_mut().for_each(|item| {
        if let ImplItem::Method(m) = item {
            m.attrs
                .retain(|a| a.path.get_ident() != Some(&format_ident!("builder")));
        }
    });
}
