use buildstructor::buildstructor;
pub struct Foo {
    simple: String,
}

/// Some other docs
#[buildstructor]
impl Foo {
    /// Test docs
    #[builder]
    fn new(simple: String) -> Foo {
        Self { simple }
    }
}

/// Some other docs
#[derive(buildstructor::Builder)]
pub struct Bar {
    /// Test docs
    simple: String,
}

fn main() {
    let _ = Foo::builder().simple("3").build();
    let _ = Foo::builder().simple("3".to_string()).build();
}
