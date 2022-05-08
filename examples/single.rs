use buildstructor::builder;

pub struct Single {
    simple: usize,
}

#[builder]
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
