use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct WS
    trait WSImpl ( ( {' ' '\t' '\n' '\r'}* ) )
}

impl WSImpl for WS {}
