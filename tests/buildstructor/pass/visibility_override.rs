mod sub1 {
    pub(crate) struct Foo {
        simple: Bar1,
    }

    pub(crate) struct Bar1 {
        pub(crate) simple: usize,
    }

    #[buildstructor::buildstructor]
    impl Foo {
        #[builder(visibility = "pub (crate)")]
        fn new(simple: Bar1) -> Self {
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

    #[buildstructor::buildstructor]
    impl Foo {
        #[builder(visibility = "pub")]
        fn new(simple: Bar2) -> Self {
            Self { simple }
        }
    }
}

fn main() {
    let _ = sub1::Foo::builder()
        .simple(sub1::Bar1 { simple: 1 })
        .build();
    let _ = sub2::Foo::builder()
        .simple(sub2::Bar2 { simple: 2 })
        .build();
}
