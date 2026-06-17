use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub enum JSONBoolean
    trait JSONBooleanImpl {
        True ( "true" )
        False ( "false" )
    }
}

impl JSONBooleanImpl for JSONBoolean {}
