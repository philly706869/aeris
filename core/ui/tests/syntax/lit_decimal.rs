use aeris_ui::syntax;

syntax! {
    pub LitDecimal
    LitDecimal -> value:Body
    Body -> Digit+ ('_' Digit+)*
    Digit -> {'0' '1' '2' '3' '4' '5' '6' '7' '8' '9'}
}

pub struct LitDecimal(String);
