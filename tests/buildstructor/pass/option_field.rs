use buildstructor::builder;
pub struct Foo {
    simple: Option<usize>,
}

#[builder]
impl Foo {
    fn new(simple: Option<usize>) -> Foo {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple(3).build();
    let _ = Foo::builder().and_simple(Some(3)).build();
    let _ = Foo::builder().build();
}
