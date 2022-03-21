use buildstructor::builder;
pub struct Foo {
    simple: usize,
    simple2: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize, simple2: usize) -> Foo {
        Self { simple, simple2 }
    }
}

fn main() {
    let _ = Foo::builder().simple(2).simple2(3).build();
}
