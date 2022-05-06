mod sub1 {
    pub(crate) struct Foo {
        simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        pub(crate) fn new(simple: usize) -> Self {
            Self { simple }
        }
    }
}

mod sub2 {
    pub struct Foo {
        simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        pub fn new(simple: usize) -> Self {
            Self { simple }
        }
    }
}

mod sub3 {
    struct Foo {
        simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        fn new(simple: usize) -> Self {
            Self { simple }
        }
    }

    pub fn foo() {
        let _ = Foo::builder().simple(3).build();
    }
}

fn main() {
    let _ = sub1::Foo::builder().simple(3).build();
    let _ = sub2::Foo::builder().simple(3).build();
    sub3::foo();
}
