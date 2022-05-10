use buildstructor::buildstructor;
pub struct Exchange {
    location: String,
}

#[buildstructor]
impl Exchange {
    #[builder]
    fn hmm() -> Exchange {
        Self {
            location: "Fakenham".to_string(),
        }
    }
    #[builder]
    fn do_phone_call(&self, _number: String) {}
}

fn main() {}
