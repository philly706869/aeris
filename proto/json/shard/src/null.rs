use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct JSONNull
    trait JSONNullImpl ( "null" )
}

impl JSONNullImpl for JSONNull {}
