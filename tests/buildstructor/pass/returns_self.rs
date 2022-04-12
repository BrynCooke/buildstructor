use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize) -> Self {
        Self { simple }
    }
}

fn main() {
    let _: Foo = Foo::builder().simple(3).build();
}
