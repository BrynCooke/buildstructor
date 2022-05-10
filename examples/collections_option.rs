use std::collections::HashMap;

use buildstructor::buildstructor;

pub struct Collections {
    map: HashMap<Option<String>, Option<String>>,
}

#[buildstructor]
impl Collections {
    #[builder]
    fn new(map: HashMap<Option<String>, Option<String>>) -> Collections {
        Self { map }
    }
}

fn main() {
    let collections = Collections::builder()
        .map_entry(Some("Appa".to_string()), Some("1".to_string()))
        .map_entry(Some("Momo".to_string()), Some("2".to_string()))
        .build();
    assert_eq!(
        collections.map,
        HashMap::from([
            (Some("Appa".to_string()), Some("1".to_string())),
            (Some("Momo".to_string()), Some("2".to_string()))
        ])
    );
}
