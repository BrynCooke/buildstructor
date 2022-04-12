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
            #[builder]
            impl Foo {
                fn new(simple: usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn pub_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
                pub fn new(simple: usize) -> Foo {
                    Self { simple }
                }
            }
        )
    }

    pub fn multi_field_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
                fn new(simple: usize, simple2: usize) -> Foo {
                    Self { simple, simple2 }
                }
            }
        )
    }

    pub fn fallible_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
                fn new(simple: usize) -> Result<Foo, String> {
                    Ok(Self { simple })
                }
            }
        )
    }

    pub fn async_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
                async fn new(simple: usize) -> Foo {
                    Foo { simple }
                }
            }
        )
    }

    pub fn generic_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl<T> Foo<T> {
                fn new(simple: T) -> Foo<T> {
                    Self { simple }
                }
            }
        )
    }

    pub fn into_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
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
            #[builder]
            impl Foo {
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

    pub fn option_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
                fn new(option: Option<usize>) -> Foo {
                    Foo { option }
                }
            }
        )
    }

    pub fn collections_test_case() -> Ast {
        parse_quote!(
            #[builder]
            impl Foo {
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
            #[builder]
            impl Foo {
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
            #[builder]
            impl Foo {
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
            #[builder]
            impl Foo {
                fn new(simple: usize) -> Self {
                    Self { simple }
                }
            }
        )
    }
}
