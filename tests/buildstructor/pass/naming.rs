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
    #[builder(entry = "phone", exit = "call")]
    fn do_phone_call(&self, _number: String) {}
}

fn main() {
    let _ = Exchange::builder().location("Fakenham").build();
    let exchange = Exchange::fake_builder().build();
    exchange.phone().number("01328 286286").call();
}
