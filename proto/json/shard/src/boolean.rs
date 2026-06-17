use aeris::ui::cluster;

cluster! {
    pub enum JSONBoolean
    trait JSONBooleanImpl {
        True ( "true" )
        False ( "false" )
    }
}

impl JSONBooleanImpl for JSONBoolean {}
