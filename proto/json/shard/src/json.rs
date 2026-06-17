use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct JSON
    trait JSONImpl {
        (ws): WS
        value: JSONValue
        (ws): WS
    }
}

impl JSONImpl for JSON {}
