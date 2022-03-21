use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple(3).build();
}
