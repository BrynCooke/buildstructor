use buildstructor::builder;
pub struct Foo {
    simple: String,
}

#[builder]
impl Foo {
    fn new<T>(simple: T) -> Foo
    where
        T: Into<String>,
    {
        Self {
            simple: simple.into(),
        }
    }
}

fn main() {
    let _ = Foo::builder().simple("2").build();
}
