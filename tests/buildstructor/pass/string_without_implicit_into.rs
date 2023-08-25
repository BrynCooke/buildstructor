use buildstructor::buildstructor;
pub struct Foo {
    simple: String,
}

#[buildstructor]
impl Foo {
    #[builder(with_into = false)]
    fn new(simple: String) -> Self {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("foo".to_string()).build();
}
