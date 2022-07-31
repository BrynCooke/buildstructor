#[derive(buildstructor::Builder)]
pub struct Single {
    simple: usize,
}

mod sub {
    #[derive(buildstructor::Builder)]
    pub struct Generic<T> {
        pub simple: T,
    }
}

fn main() {
    let single = Single::builder().simple(2).build();
    assert_eq!(single.simple, 2);

    let generic = sub::Generic::builder().simple(2).build();
    assert_eq!(generic.simple, 2);
}
