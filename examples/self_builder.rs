use buildstructor::buildstructor;

#[derive(Default)]
pub struct Client;

#[buildstructor]
impl Client {
    #[builder(entry = "message", exit = "send")]
    fn call_with_no_return(self, _simple: String) {}

    #[builder(entry = "message_ref", exit = "send")]
    fn call_with_no_return_ref(&self, _simple: String) {}

    #[builder(entry = "message_ref_mut", exit = "send")]
    fn call_with_no_return_ref_mut(&mut self, _simple: String) {}

    #[builder(entry = "query", exit = "call")]
    fn call_with_return(self, _simple: String) -> bool {
        true
    }

    #[builder(entry = "query_ref", exit = "call")]
    fn call_with_return_ref(&self, _simple: String) -> bool {
        true
    }

    #[builder(entry = "query_ref_mut", exit = "call")]
    fn call_with_return_ref_mut(&mut self, _simple: String) -> bool {
        true
    }
}

fn main() {
    Client.message().simple("3".to_string()).send();
    Client.query().simple("3".to_string()).call();

    let client = Client;
    client.message_ref().simple("3".to_string()).send();
    client.query_ref().simple("3".to_string()).call();

    let mut client = Client;
    client.message_ref_mut().simple("3".to_string()).send();
    client.query_ref_mut().simple("3".to_string()).call();
}
