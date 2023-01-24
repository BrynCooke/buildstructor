#![warn(missing_docs)]
//! Doc
use buildstructor::{buildstructor, Builder};

/// Doc
pub struct Single {
    simple1: usize,
    simple2: usize,
}

#[buildstructor]
impl Single {
    /// Method description 1
    /// Method description 2
    ///
    ///
    /// # Arguments
    ///
    /// * `simple1`: SimpleArg1 Line1
    ///              SimpleArg1 Line2
    /// * `simple2`: SimpleArg2 Line1
    ///              SimpleArg2 Line2
    ///
    ///
    /// returns: Single
    /// # Examples
    /// ```
    ///
    /// ```
    #[builder]
    pub fn new(simple1: usize, simple2: usize) -> Single {
        Self { simple1, simple2 }
    }
}

/// Doc
#[derive(Builder)]
pub struct Double {
    /// Simple 1
    simple1: usize,

    /// Simple 2
    simple2: Option<usize>,
}

fn main() {
    let single = Single::builder().simple1(2).simple2(2).build();
    assert_eq!(single.simple1, 2);
    assert_eq!(single.simple2, 2);
    let double = Double::builder().simple1(1).simple2(2).build();
    assert_eq!(double.simple1, 1);
    assert_eq!(double.simple2.unwrap_or_default(), 2);
}
