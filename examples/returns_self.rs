use buildstructor::builder;

pub struct ReturnsSelf {
    simple: usize,
}

#[builder]
impl ReturnsSelf {
    fn new(simple: usize) -> Self {
        Self { simple }
    }
}

fn main() {
    let generic = ReturnsSelf::builder().simple(2).build();
    assert_eq!(generic.simple, 2);
}
