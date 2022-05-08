mod sub1 {
    pub(crate) struct Foo {
        simple: Bar1,
    }

    pub(crate) struct Bar1 {
        pub(crate) simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        #[builder]
        pub(crate) fn new(simple: Bar1) -> Self {
            Self { simple }
        }
    }
}

mod sub2 {
    pub struct Foo {
        simple: Bar2,
    }

    pub struct Bar2 {
        pub(crate) simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        #[builder]
        pub fn new(simple: Bar2) -> Self {
            Self { simple }
        }
    }
}

mod sub3 {
    struct Foo {
        simple: Bar3,
    }

    pub struct Bar3 {
        pub simple: usize,
    }

    #[buildstructor::builder]
    impl Foo {
        #[builder]
        fn new(simple: Bar3) -> Self {
            Self { simple }
        }
    }

    pub fn foo() {
        let _ = Foo::builder().simple(Bar3 { simple: 3 }).build();
    }
}

fn main() {
    let _ = sub1::Foo::builder()
        .simple(sub1::Bar1 { simple: 1 })
        .build();
    let _ = sub2::Foo::builder()
        .simple(sub2::Bar2 { simple: 2 })
        .build();
    sub3::foo();
}
