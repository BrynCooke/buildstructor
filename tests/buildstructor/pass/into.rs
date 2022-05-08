use buildstructor::builder;
pub struct Foo {
    simple: String,
}

#[builder]
impl Foo {
    #[builder]
    fn new<T: Into<String>>(simple: T) -> Foo {
        Self {
            simple: simple.into(),
        }
    }
}

fn main() {
    let _ = Foo::builder().simple("2").build();
}
