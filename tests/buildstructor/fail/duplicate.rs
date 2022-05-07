use buildstructor::builder;
pub struct Foo {
    simple: String,
}

#[builder]
impl Foo {
    fn new(simple: String) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("3").simple("3").build();
}
