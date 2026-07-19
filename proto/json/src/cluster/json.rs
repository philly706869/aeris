use aeris::ui::cluster;

use crate::cluster::{JSONValue, WS};

cluster! {
    #[derive(Debug)]
    pub struct JSON
    trait JSONImpl {
        ws: WS
        value: JSONValue
        ws: WS
    }
}

impl JSONImpl for JSON {}
