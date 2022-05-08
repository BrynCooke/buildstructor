use buildstructor::builder;

pub struct IntoType {
    simple: String,
}

#[builder]
impl IntoType {
    #[builder]
    fn new<T: Into<String>>(simple: T) -> IntoType {
        IntoType {
            simple: simple.into(),
        }
    }
}

fn main() {
    let into = IntoType::builder().simple("hi").build();
    assert_eq!(into.simple, "hi");
}
