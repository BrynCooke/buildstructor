use buildstructor::builder;
use std::collections::{HashMap, HashSet};

pub struct Foo {
    names: HashSet<String>,
    ages: HashMap<String, u64>,
    addresses: Vec<String>,
}

#[builder]
impl Foo {
    fn new(names: HashSet<String>, ages: HashMap<String, u64>, addresses: Vec<String>) -> Foo {
        Self {
            names,
            ages,
            addresses,
        }
    }
}

fn main() {
    let _ = Foo::builder()
        .name("Nandor".to_string())
        .name("Nandor".to_string())
        .name("Colin".to_string())
        .names(HashSet::from(["Nadja", "Laszlo"].map(str::to_string)))
        .age("Nandor".to_string(), 0)
        .age("Nandor".to_string(), 759)
        .age("Colin".to_string(), 100)
        .ages(HashMap::from(
            [("Nadja", 650), ("Laszlo", 364)].map(|(k, v)| (k.to_string(), v)),
        ))
        .address("Staten Island".to_string())
        .address("Staten Island".to_string())
        .addresses(Vec::from(["France", "Turkey"].map(str::to_string)))
        .build();
}
