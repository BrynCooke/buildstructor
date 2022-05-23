use buildstructor::buildstructor;
pub struct Foo<'a> {
    simple: &'a String,
}

#[buildstructor]
impl<'a> Foo<'a> {
    #[builder]
    fn new(simple: &'a String) -> Foo<'a> {
        Self { simple }
    }
}

fn main() {
    let f = "3".to_string();
    let _ = Foo::builder().simple(&f).build();
}
