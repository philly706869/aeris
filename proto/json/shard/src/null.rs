use aeris::ui::cluster;

cluster! {
    pub struct JSONNull
    trait JSONNullImpl ( "null" )
}

impl JSONNullImpl for JSONNull {}
