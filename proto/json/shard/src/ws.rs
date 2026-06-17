use aeris::ui::cluster;

cluster! {
    pub struct WS
    trait WSImpl ( ( {' ' '\t' '\n' '\r'}* ) )
}

impl WSImpl for WS {}
