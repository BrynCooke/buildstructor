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
    #[builder]
    fn random(simple: String) -> Foo {
        Self { simple }
    }
    #[builder]
    fn fake_new(simple: String) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple("3").build();
    let _ = Foo::random_builder().simple("3").build();
    let _ = Foo::fake_builder().simple("3").build();
}
