use buildstructor::buildstructor;

pub struct ReturnsSelf {
    simple: usize,
}

#[buildstructor]
impl ReturnsSelf {
    #[builder]
    fn new(simple: usize) -> Self {
        Self { simple }
    }
}

fn main() {
    let generic = ReturnsSelf::builder().simple(2).build();
    assert_eq!(generic.simple, 2);
}
