struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload,
        }
    }
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    /*
    clone and to_owned method:
     - .clone() returns its receiver => clone() on a &str returns a &str.
       ("For most types, clone() is sufficient because it's only defined on the underlying type and not on the reference type)
     - If you want a String, you need a different method, which in this case is .to_owned().
    */
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );
    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
    cmd1.print_payload();
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

fn serialize_payload<T>(payload: &T) -> String {
    // convert payload to JSON string...
    "placeholder".to_owned()
}
