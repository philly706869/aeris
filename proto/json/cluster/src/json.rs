use aeris::ui::cluster;

use crate::{JSONValue, WS};

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
