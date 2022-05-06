use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
    fn self_new(simple: usize) -> Result<Self, String> {
        Ok(Self { simple })
    }
    fn deep_self_new(simple: usize) -> Result<Result<Self, String>, String> {
        Ok(Ok(Self { simple }))
    }
}

fn main() {
    let _ = Foo::builder().simple(2).build().is_ok();
    let _ = Foo::self_builder().simple(2).build().is_ok();
    let _ = Foo::deep_self_builder().simple(2).build().is_ok();
}
