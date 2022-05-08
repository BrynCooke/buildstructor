use buildstructor::buildstructor;
pub struct Foo<T> {
    simple: T,
}

#[buildstructor]
impl<T> Foo<T> {
    #[builder]
    fn new(simple: T) -> Foo<T> {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple(3).build();
}
