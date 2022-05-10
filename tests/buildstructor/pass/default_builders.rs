use buildstructor::buildstructor;
pub struct Exchange {
    location: String,
}

#[buildstructor]
impl Exchange {
    #[builder]
    fn new(location: String) -> Exchange {
        Self { location }
    }

    #[builder]
    fn fake_new() -> Exchange {
        Self {
            location: "Fakenham".to_string(),
        }
    }
}

fn main() {
    let _ = Exchange::builder().location("Fakenham").build();
    let _ = Exchange::fake_builder().build();
}
