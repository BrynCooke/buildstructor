use buildstructor::builder;

use std::error::Error;

pub struct Fallible {
    simple: usize,
}

#[builder]
impl Fallible {
    fn try_new(simple: usize) -> Result<Fallible, Box<dyn Error>> {
        Ok(Self { simple })
    }
}

fn main() {
    let fallible = Fallible::try_builder().simple(2).build().unwrap();
    assert_eq!(fallible.simple, 2);
}