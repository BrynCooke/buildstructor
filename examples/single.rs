use buildstructor::buildstructor;

pub struct Single {
    simple: usize,
}

#[buildstructor]
impl Single {
    #[builder]
    fn new(simple: usize) -> Single {
        Self { simple }
    }
}

fn main() {
    let single = Single::builder().simple(2).build();
    assert_eq!(single.simple, 2);
}
