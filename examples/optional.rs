use buildstructor::buildstructor;

pub struct Optional {
    simple: Option<usize>,
}

#[buildstructor]
impl Optional {
    #[builder]
    fn new(simple: Option<usize>) -> Optional {
        Self { simple }
    }
}

#[tokio::main]
async fn main() {
    let optional = Optional::builder().simple(2).build();
    assert_eq!(optional.simple, Some(2));
    let optional = Optional::builder().and_simple(Some(2)).build();
    assert_eq!(optional.simple, Some(2));
    let optional = Optional::builder().build();
    assert_eq!(optional.simple, None);
}
