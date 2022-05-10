use buildstructor::builder;
pub struct Foo {
    simple: String,
}

#[builder]
impl Foo {
    fn new(simple: String) -> Foo {
        Self { simple }
    }
    fn fake_new(simple: String) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("3").build();
    let _ = Foo::fake_builder().simple("3").build();
}
