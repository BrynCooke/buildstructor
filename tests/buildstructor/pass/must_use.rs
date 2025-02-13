use buildstructor::buildstructor;
pub struct Foo {
    simple: String,
}

#[buildstructor]
impl Foo {
    #[builder]
    fn new(simple: String) -> Foo {
        Self { simple }
    }
}

fn main() {
    Foo::builder().simple("3");
}
