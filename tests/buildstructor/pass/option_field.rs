use buildstructor::buildstructor;
pub struct Foo {
    simple: Option<String>,
}

#[buildstructor]
impl Foo {
    #[builder]
    fn new(simple: Option<String>) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("3").build();
    let _ = Foo::builder().simple("3".to_string()).build();
    let _ = Foo::builder().and_simple(Some("3")).build();
    let _ = Foo::builder().and_simple(Some("3".to_string())).build();
    let _ = Foo::builder().build();
}
