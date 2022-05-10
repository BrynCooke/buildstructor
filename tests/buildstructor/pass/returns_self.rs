use buildstructor::buildstructor;
pub struct Foo {
    simple: usize,
}

#[buildstructor]
impl Foo {
    #[builder]
    fn new(simple: usize) -> Self {
        Self { simple }
    }
}

fn main() {
    let _: Foo = Foo::builder().simple(3).build();
}
