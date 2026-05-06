use aeris_ui::syntax;

syntax! {
    pub LitStr
    LitStr -> '"' Chars '"'
    Chars -> (Char | EscapeChar)*
    Char -> {! '"' '\\'}
}

pub struct LitStr(String);
