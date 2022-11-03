use buildstructor::buildstructor;

pub trait MyTrait {
    type Bar;
}

struct MyTraitImpl {}

impl MyTrait for MyTraitImpl {
    type Bar = String;
}

#[derive(Debug)]
pub struct Foo<T: MyTrait> {
    foo: T,
    bar: T::Bar,
}

#[buildstructor]
impl<T: MyTrait> Foo<T> {
    #[builder]
    pub fn new(foo: T, bar: T::Bar) -> Foo<T> {
        Foo { foo, bar }
    }
}

fn main() {
    let _ = Foo::builder().foo(MyTraitImpl {}).bar("hi").build();
}
