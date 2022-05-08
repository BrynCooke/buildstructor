use buildstructor::builder;
pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    #[builder]
    fn try_new(simple: usize) -> Result<Foo, String> {
        Ok(Self { simple })
    }
}

fn main() {
    let _ = Foo::try_builder().simple(2).build().is_ok();
}
