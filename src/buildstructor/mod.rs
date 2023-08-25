pub mod analyze;
pub mod codegen;
pub mod lower;
pub mod parse;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::parse::Ast;
    use syn::parse_quote;

    pub fn single_field_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(simple: usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn pub_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                pub fn new(simple: usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn multi_field_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(simple: usize, simple2: usize) -> Foo {
                    Self { simple, simple2 }
                }
            }
        )
    }

    pub fn fallible_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(simple: usize) -> Result<Foo, String> {
                    Ok(Self { simple })
                }
            }
        )
    }

    pub fn async_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                async fn new(simple: usize) -> Foo {
                    Foo { simple }
                }
            }
        )
    }

    pub fn generic_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl<T> Foo<T> {
                #[builder]
                fn new(simple: T) -> Foo<T> {
                    Self { simple }
                }
            }
        )
    }

    pub fn into_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new<T: Into<String>>(simple: T) -> Foo {
                    Foo {
                        simple: simple.into(),
                    }
                }
            }
        )
    }

    pub fn into_where_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new<T>(simple: T) -> Foo
                where
                    T: Into<String>,
                {
                    Foo {
                        simple: simple.into(),
                    }
                }
            }
        )
    }

    pub fn string_without_implicit_into_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder(with_into = false)]
                pub fn new(foo: String) -> Self {
                    Foo { foo }
                }
            }
        )
    }

    pub fn option_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(option: Option<usize>) -> Foo {
                    Foo { option }
                }
            }
        )
    }

    pub fn collections_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(
                    simple: usize,
                    set: HashSet<String>,
                    map: HashMap<String, String>,
                    vec: Vec<String>,
                    btmap: BTreeMap<String, String>,
                    btset: BTreeSet<String>,
                ) -> Foo {
                    Self {
                        simple,
                        set,
                        map,
                        vec,
                        btmap,
                        btset,
                    }
                }
            }
        )
    }

    pub fn collections_generics_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new<K: Into<String> + Eq + Hash, V: Into<String>>(param: HashMap<K, V>) -> Foo {
                    Self {
                        param: param
                            .into_iter()
                            .map(|(k, v)| (k.into(), v.into()))
                            .collect(),
                    }
                }
            }
        )
    }

    pub fn collections_option_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(param: HashMap<Option<String>, Option<String>>) -> Foo {
                    Self {
                        param: param
                            .into_iter()
                            .map(|(k, v)| (k.into(), v.into()))
                            .collect(),
                    }
                }
            }
        )
    }

    pub fn returns_self_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                #[builder]
                fn new(simple: usize) -> Self {
                    Self { simple }
                }
            }
        )
    }

    pub fn multiple_generics_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl<T> Request<T> {
                #[builder]
                pub fn fake_new<K, V>(
                    headers: Vec<(K, V)>,
                    uri: Option<http::Uri>,
                    method: Option<http::Method>,
                    body: T,
                ) -> http::Result<Request<T>>
                where
                    HeaderName: TryFrom<K>,
                    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
                    HeaderValue: TryFrom<V>,
                    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
                {
                    let mut builder = http::request::Builder::new();
                    for (key, value) in headers {
                        builder = builder.header(key, value);
                    }
                    let req = builder.body(body)?;

                    Ok(Self { inner: req })
                }
            }
        )
    }

    pub fn collections_generics_test_case2() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Collections {
                #[builder]
                fn new<K: Into<String> + Eq + Hash, V: Into<String>>(
                    map: HashMap<K, V>,
                    set: HashSet<K>,
                ) -> Collections {
                    Self {
                        map: map.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                        set: set.into_iter().map(|v| v.into()).collect(),
                    }
                }
            }
        )
    }

    pub fn self_receiver_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Client {
                #[builder(entry = "message", exit = "send")]
                fn call_with_no_return(self, simple: String) {}

                #[builder(entry = "message_ref", exit = "send")]
                fn call_with_no_return_ref(&self, simple: String) {}

                #[builder(entry = "query", exit = "call")]
                fn call_with_return(self, simple: String) -> bool {
                    true
                }

                #[builder(entry = "query_ref", exit = "call")]
                fn call_with_return_ref(&self, simple: String) -> bool {
                    true
                }
            }
        )
    }

    pub fn doc_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                /// Test doc
                #[builder]
                fn new(simple: usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn reference_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo {
                /// Test doc
                #[builder]
                fn new(simple: &usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn self_reference_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Client {
                #[builder(entry = "builder")]
                fn new(&self) {}
            }
        )
    }

    pub fn lifetime_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl<'a> Foo<'a> {
                #[builder]
                fn new(simple: &'a String) -> Foo<'a> {
                    Self { simple }
                }
            }
        )
    }

    pub fn specialization_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo<usize> {
                #[builder]
                fn bound_new(simple: usize) -> Foo<usize> {
                    Self { simple }
                }
            }
        )
    }

    pub fn specialization_returns_self_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl Foo<usize> {
                #[builder]
                fn bound_new(simple: usize) -> Self {
                    Self { simple }
                }
            }
        )
    }

    pub fn associated_types_test_case() -> Ast {
        parse_quote!(
            #[buildstructor]
            impl<T: MyTrait> Foo<T> {
                #[builder]
                pub fn new(foo: T, bar: T::Bar) -> Foo<T> {
                    Foo { foo, bar }
                }
            }
        )
    }
}
