use buildstructor::buildstructor;
pub struct Foo {
    simple: String,
}

#[buildstructor]
impl Foo {
    #[builder(unknown = "unknown")]
    fn new(simple: String) -> Self {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("foo").build();
}
