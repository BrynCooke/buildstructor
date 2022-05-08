use buildstructor::builder;

pub struct Foo {
    simple: usize,
}

#[builder]
impl Foo {
    #[builder]
    async fn new(simple: usize) -> Foo {
        Foo { simple }
    }
}

#[tokio::main]
async fn main() {
    let _ = Foo::builder().simple(3).build().await;
}
