use buildstructor::builder;
pub struct Foo {
    simple: String,
}

#[builder]
impl Foo {
    #[builder]
    fn new(simple: String) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().build();
}
