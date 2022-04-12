use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
    fn try_new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
    fn maybe_new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
}

fn main() {
    let _ = Foo::builder().simple(2).build().is_ok();
    let _ = Foo::try_builder().simple(2).build().is_ok();
    let _ = Foo::maybe_builder().simple(2).build().is_ok();
}
