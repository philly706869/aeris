mod array;
mod boolean;
mod null;
mod number;
mod object;
mod string;
mod value;
mod ws;

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
