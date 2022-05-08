use buildstructor::builder;

pub struct Generic<T> {
    simple: T,
}

#[builder]
impl<T> Generic<T> {
    #[builder]
    fn new(simple: T) -> Generic<T> {
        Self { simple }
    }
}

fn main() {
    let generic = Generic::builder().simple(2).build();
    assert_eq!(generic.simple, 2);
}
