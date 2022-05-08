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
    let _ = Foo::builder().simple("3").build();
    let _ = Foo::builder().simple("3".to_string()).build();
}
