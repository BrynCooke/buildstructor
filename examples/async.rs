use buildstructor::builder;

pub struct Async {
    simple: usize,
}

#[builder]
impl Async {
    #[builder]
    async fn new(simple: usize) -> Async {
        Self { simple }
    }
}

#[tokio::main]
async fn main() {
    let asc = Async::builder().simple(2).build().await;
    assert_eq!(asc.simple, 2);
}
