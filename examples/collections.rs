use std::collections::{HashMap, HashSet};

use buildstructor::builder;

pub struct Collections {
    names: HashSet<String>,
    ages: HashMap<String, u64>,
    addresses: Vec<String>,
}

#[builder]
impl Collections {
    fn new(
        names: HashSet<String>,
        ages: HashMap<String, u64>,
        addresses: Vec<String>,
    ) -> Collections {
        Self {
            names,
            ages,
            addresses,
        }
    }
}

fn main() {
    let collections = Collections::builder()
        .name("Nandor".to_string())
        .name("Nandor")
        .name("Colin".to_string())
        .names(HashSet::from(["Nadja", "Laszlo"].map(str::to_string)))
        .age("Nandor".to_string(), 0)
        .age("Nandor", 759)
        .age("Colin".to_string(), 100)
        .ages(HashMap::from(
            [("Nadja", 650), ("Laszlo", 364)].map(|(k, v)| (k.to_string(), v)),
        ))
        .address("Staten Island".to_string())
        .address("Staten Island")
        .addresses(Vec::from(["France", "Turkey"].map(str::to_string)))
        .build();

    assert_eq!(
        collections.names,
        HashSet::from(["Nandor", "Laszlo", "Nadja", "Colin"].map(str::to_string))
    );
    assert_eq!(
        collections.ages,
        HashMap::from(
            [
                ("Nadja", 650),
                ("Laszlo", 364),
                ("Colin", 100),
                ("Nandor", 759)
            ]
            .map(|(k, v)| (k.to_string(), v))
        )
    );
    assert_eq!(
        collections.addresses,
        Vec::from(["Staten Island", "Staten Island", "France", "Turkey"].map(str::to_string))
    );
}
