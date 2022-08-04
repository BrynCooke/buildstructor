#![doc = include_str!("../README.md")]
extern crate core;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{parse2, parse_quote, Attribute, Data, DeriveInput, ImplItem};
mod buildstructor;
use crate::buildstructor::analyze;
use crate::buildstructor::codegen;
use crate::buildstructor::lower;
use crate::buildstructor::parse;
use crate::parse::Ast;

/// Derive a builder from a constructor!
///
/// 1. Import the `buildstructor` macro.
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
    do_buildstructor(false, attr, item)
}

#[proc_macro_attribute]
#[deprecated(
    since = "0.2.0",
    note = "#[buildstructor::builder] should be migrated to #[buildstructor::buildstructor] and individual methods annotated with #[builder]"
)]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    do_buildstructor(true, attr, item)
}

/// Derive a builder AND a constructor!
///
/// 1. Import the `Builder` macro.
/// 2. Use your automatically derived builder.
///
/// TLDR: Write your Rust constructors as you would normally, and get a generated builder.
///
/// # Examples
///
/// ```rust
/// use buildstructor::Builder;
///
/// #[derive(Builder)]
/// struct MyStruct {
///     sum: usize,
/// }
///
/// # #[allow(clippy::needless_doctest_main)]
/// # fn main() {
///   let mine = MyStruct::builder().sum(3).build();
///   assert_eq!(mine.sum, 3);
/// # }
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    do_derive(item)
}

fn do_buildstructor(
    legacy_default_builders: bool,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let attr = quote::quote! {#[buildstructor(#attr)]};
    match parse::parse(TokenStream::from_iter(vec![attr.into(), item.clone()]).into())
        .map_err(|e| e.into_compile_error())
    {
        Ok(mut ast) => {
            // We have the AST, we can return the token stream regardless of if there was success or not as long as we sanitize it of helper attributes.
            let mut results: Vec<proc_macro::TokenStream> =
                match analyze::analyze(legacy_default_builders, &ast)
                    .map_err(|e| e.into_compile_error())
                {
                    Ok(builders) => builders
                        .into_iter()
                        .map(|builder| match builder {
                            Ok(builder) => {
                                let ir =
                                    lower::lower(builder).map_err(|e| e.into_compile_error())?;
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

            // Relax clippy on constructors
            allow_many_params(&mut ast);

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

fn allow_many_params(ast: &mut Ast) {
    let allow_params: Attribute =
        parse_quote!(#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]);
    ast.item.items.iter_mut().for_each(|item| {
        if let ImplItem::Method(m) = item {
            if m.attrs
                .iter()
                .any(|attr| attr.path.get_ident() == Some(&format_ident!("builder")))
            {
                m.attrs.push(allow_params.clone())
            }
        }
    });
}

fn sanitize(ast: &mut Ast) {
    ast.item.items.iter_mut().for_each(|item| {
        if let ImplItem::Method(m) = item {
            m.attrs
                .retain(|a| a.path.get_ident() != Some(&format_ident!("builder")));
        }
    });
}

pub(crate) fn do_derive(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse2(item.into()).unwrap();
    let vis = &input.vis.to_token_stream().to_string();
    let self_ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    if let Data::Struct(s) = &input.data {
        let parameters: Vec<TokenStream2> = s
            .fields
            .iter()
            .map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote::quote! {
                    #name : #ty
                }
            })
            .collect();

        let fields: Vec<&Option<Ident>> = s.fields.iter().map(|f| &f.ident).collect();

        quote::quote! {
            #[buildstructor::buildstructor]
            impl #impl_generics #self_ty #ty_generics #where_clause {
                #[builder(visibility=#vis)]
                fn new(
                    #(#parameters),*
                )->#self_ty #ty_generics{
                    Self {
                        #(#fields),*
                    }
                }
            }

        }
        .into()
    } else {
        syn::Error::new(input.span(), "derive(Builder) can only be used on structs")
            .into_compile_error()
            .into()
    }
}
