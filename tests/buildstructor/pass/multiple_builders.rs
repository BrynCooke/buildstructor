use buildstructor::builder;
pub struct Foo1 {
    simple: usize,
}

pub struct Foo2 {
    simple: usize,
}

#[builder]
impl Foo1 {
    #[builder]
    fn new(simple: usize) -> Foo1 {
        Self { simple }
    }
}

#[builder]
impl Foo2 {
    #[builder]
    fn new(simple: usize) -> Foo2 {
        Self { simple }
    }
}

fn main() {
    let _ = Foo1::builder().simple(3).build();
    let _ = Foo2::builder().simple(3).build();
}
