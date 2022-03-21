use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    fn new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
}

fn main() {
    let _ = Foo::builder().simple(2).build().is_ok();
}
