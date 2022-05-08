use buildstructor::buildstructor;

use std::error::Error;

pub struct Multi {
    simple: usize,
}

#[buildstructor]
impl Multi {
    #[builder]
    fn new(simple: usize) -> Multi {
        Self { simple }
    }

    #[builder]
    fn try_new(simple: usize) -> Result<Multi, Box<dyn Error>> {
        Ok(Self { simple })
    }

    #[builder]
    fn maybe_new(simple: usize) -> Option<Multi> {
        Some(Self { simple })
    }
}

fn main() {
    let regular = Multi::builder().simple(2).build();
    assert_eq!(regular.simple, 2);

    let fallible = Multi::try_builder().simple(2).build().unwrap();
    assert_eq!(fallible.simple, 2);

    let option = Multi::maybe_builder().simple(2).build().unwrap();
    assert_eq!(option.simple, 2);
}
