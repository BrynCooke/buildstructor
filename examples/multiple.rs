use buildstructor::buildstructor;

pub struct Multiple {
    simple: usize,
    simple2: usize,
}

#[buildstructor]
impl Multiple {
    #[builder]
    fn new(simple: usize, simple2: usize) -> Multiple {
        Self { simple, simple2 }
    }
}
fn main() {
    let multiple = Multiple::builder().simple(2).simple2(3).build();
    assert_eq!(multiple.simple, 2);
    assert_eq!(multiple.simple2, 3);
}
