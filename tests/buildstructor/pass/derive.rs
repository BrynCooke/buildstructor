#[derive(buildstructor::Builder)]
pub struct Single {
    simple: usize,
}

#[derive(buildstructor::Builder)]
pub struct Generic<T> {
    simple: T,
}

fn main() {
    let _ = Single::builder().simple(2).build();
    let _ = Generic::builder().simple(2).build();
}
