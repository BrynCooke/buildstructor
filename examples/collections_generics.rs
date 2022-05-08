use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use buildstructor::builder;

pub struct Collections {
    map: HashMap<String, String>,
    set: HashSet<String>,
}

#[builder]
impl Collections {
    #[builder]
    fn new<K: Into<String> + Eq + Hash, V: Into<String>>(
        map: HashMap<K, V>,
        set: HashSet<K>,
    ) -> Collections {
        Self {
            map: map.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
            set: set.into_iter().map(|v| v.into()).collect(),
        }
    }
}

fn main() {
    let collections = Collections::builder()
        .map_entry("Appa", "1")
        .map_entry("Momo", "2")
        .set_entry("Aang")
        .set_entry("Katara")
        .set_entry("Sokka")
        .set_entry("Toph")
        .build();
    assert_eq!(
        collections.map,
        HashMap::from([
            ("Appa".to_string(), "1".to_string()),
            ("Momo".to_string(), "2".to_string())
        ])
    );
    assert_eq!(
        collections.set,
        HashSet::from([
            ("Aang".to_string()),
            ("Katara".to_string()),
            ("Sokka".to_string()),
            ("Toph".to_string()),
        ])
    );
}
