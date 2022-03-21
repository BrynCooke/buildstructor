use syn::punctuated::Punctuated;
use syn::{
    AngleBracketedGenericArguments, Expr, ExprPath, ExprTuple, GenericArgument, GenericParam,
    Generics, Ident, Path, PathArguments, PathSegment, Token, TraitBound, TraitBoundModifier, Type,
    TypeParam, TypeParamBound, TypePath, TypeTuple, WhereClause,
};

pub trait IdentExt {
    fn to_generic_param(&self, ty: Option<&Type>) -> GenericParam;
    fn to_path(&self) -> Path;
    fn to_expr_path(&self) -> ExprPath;
    fn to_type_path(&self) -> TypePath;
}

impl IdentExt for Ident {
    fn to_generic_param(&self, ty: Option<&Type>) -> GenericParam {
        GenericParam::Type(TypeParam {
            attrs: Default::default(),
            ident: self.clone(),
            colon_token: Default::default(),
            bounds: ty.map_or(Punctuated::new(), |ty| {
                Punctuated::from_iter(vec![TypeParamBound::Trait(TraitBound {
                    paren_token: Default::default(),
                    modifier: TraitBoundModifier::None,
                    lifetimes: Default::default(),
                    path: ty.to_path().expect("This will only every be a path type"),
                })])
            }),
            eq_token: Default::default(),
            default: Default::default(),
        })
    }

    fn to_path(&self) -> Path {
        Path::from(PathSegment::from(self.clone()))
    }

    fn to_expr_path(&self) -> ExprPath {
        ExprPath {
            attrs: Default::default(),
            qself: Default::default(),
            path: self.to_path(),
        }
    }

    fn to_type_path(&self) -> TypePath {
        TypePath {
            qself: Default::default(),
            path: self.to_path(),
        }
    }
}

pub trait GenericsExt {
    fn to_tuple_type(&self) -> TypeTuple;
    fn to_generic_args(&self) -> AngleBracketedGenericArguments;
    fn to_expr_tuple(&self, populate: impl Fn(usize, &TypeParam) -> Expr) -> ExprTuple;
    fn without(self, idx: usize) -> Self;
    fn combine(generics: Vec<&Generics>) -> Generics;
}

impl GenericsExt for Generics {
    fn to_tuple_type(&self) -> TypeTuple {
        TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(
                self.type_params()
                    .into_iter()
                    .map(|t| Type::Path(t.ident.to_type_path())),
            )
            .with_trailing(),
        }
    }

    fn to_generic_args(&self) -> AngleBracketedGenericArguments {
        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: Punctuated::from_iter(
                self.type_params()
                    .into_iter()
                    .map(|i| GenericArgument::Type(Type::Path(i.ident.to_type_path()))),
            ),
            gt_token: Default::default(),
        }
    }

    fn to_expr_tuple(&self, populate: impl Fn(usize, &TypeParam) -> Expr) -> ExprTuple {
        ExprTuple {
            attrs: Default::default(),
            paren_token: Default::default(),
            elems: Punctuated::from_iter(
                self.type_params()
                    .into_iter()
                    .enumerate()
                    .map(|(idx, ty)| (populate)(idx, ty)),
            )
            .with_trailing(),
        }
    }

    fn without(mut self, idx: usize) -> Self {
        self.params = Punctuated::from_iter(
            self.params
                .into_iter()
                .enumerate()
                .filter(|(idx2, _)| *idx2 != idx)
                .map(|(_, p)| p),
        );
        self
    }

    fn combine(generics: Vec<&Generics>) -> Generics {
        Generics {
            params: Punctuated::from_iter(generics.iter().flat_map(|g| g.params.clone())),
            where_clause: generics.iter().filter_map(|g| g.where_clause.clone()).fold(
                None,
                |a, b| match a {
                    None => Some(b),
                    Some(a) => Some(WhereClause {
                        where_token: Default::default(),
                        predicates: Punctuated::from_iter(
                            a.predicates.iter().chain(b.predicates.iter()).cloned(),
                        ),
                    }),
                },
            ),
            ..Default::default()
        }
    }
}

pub trait AngleBracketedGenericArgumentsExt {
    fn insert(self, idx: usize, ty: Type) -> Self;
}

impl AngleBracketedGenericArgumentsExt for AngleBracketedGenericArguments {
    fn insert(mut self, idx: usize, ty: Type) -> Self {
        self.args.insert(idx, GenericArgument::Type(ty));
        self
    }
}

pub trait ExprTupleExt {
    fn with_expr(self, idx: usize, expr: Expr) -> Self;
}

impl ExprTupleExt for ExprTuple {
    fn with_expr(mut self, idx: usize, expr: Expr) -> Self {
        self.elems[idx] = expr;
        self
    }
}

pub trait TypeTupleExt {
    fn with_type(self, idx: usize, ty: Type) -> Self;
}

impl TypeTupleExt for TypeTuple {
    fn with_type(mut self, idx: usize, ty: Type) -> Self {
        self.elems[idx] = ty;
        self
    }
}

pub trait TypeExt {
    fn raw_ident(&self) -> Option<Ident>;
    fn generic_args(&self) -> Option<&Punctuated<GenericArgument, Token![,]>>;
    fn wrap_in_generic(&self, ident: Ident) -> Type;
    fn to_path(&self) -> Option<Path>;
}

impl TypeExt for Type {
    fn raw_ident(&self) -> Option<Ident> {
        if let Type::Path(path) = self {
            if path.path.leading_colon.is_none() && path.path.segments.len() == 1 {
                Some(path.path.segments[0].ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn generic_args(&self) -> Option<&Punctuated<GenericArgument, Token![,]>> {
        if let Type::Path(path) = self {
            if path.path.leading_colon.is_none() && path.path.segments.len() == 1 {
                if let PathArguments::AngleBracketed(args) = &path.path.segments[0].arguments {
                    return Some(&args.args);
                }
            }
        }
        None
    }

    fn wrap_in_generic(&self, ident: Ident) -> Type {
        Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: Default::default(),
                        lt_token: Default::default(),
                        args: Punctuated::from_iter(vec![GenericArgument::Type(self.clone())]),
                        gt_token: Default::default(),
                    }),
                }]),
            },
        })
    }

    fn to_path(&self) -> Option<Path> {
        if let Type::Path(path) = self {
            return Some(path.path.clone());
        }
        None
    }
}

pub trait PunctuatedExt<T, P> {
    fn with_trailing(self) -> Self;
}

impl<T, P: Default> PunctuatedExt<T, P> for Punctuated<T, P> {
    fn with_trailing(mut self) -> Self {
        if !self.is_empty() && !self.trailing_punct() {
            self.push_punct(P::default())
        }
        self
    }
}
