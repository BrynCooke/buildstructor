use buildstructor::buildstructor;
pub struct Exchange {
    location: String,
}

#[buildstructor(default_builders = true)]
impl Exchange {
    fn new(location: String) -> Exchange {
        Self { location }
    }

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
