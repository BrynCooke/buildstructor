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

    #[builder]
    fn bound2_new(simple: T) -> Foo<T>
    where
        T: std::fmt::Debug,
    {
        Self { simple }
    }
}

#[buildstructor]
impl<T: std::fmt::Debug> Foo<T> {
    #[builder]
    fn bound1_new(simple: T) -> Foo<T> {
        Self { simple }
    }
}

#[buildstructor]
impl Foo<usize> {
    #[builder]
    fn bound3_new(simple: usize) -> Foo<usize> {
        Self { simple }
    }
}

fn main() {
    let _ = Foo::builder().simple(3).build();
    let _ = Foo::bound1_builder().simple(3).build();
    let _ = Foo::bound2_builder().simple(3).build();
    let _ = Foo::bound3_builder().simple(3).build();
}
