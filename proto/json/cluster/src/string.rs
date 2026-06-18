use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct JSONString
    trait JSONStringImpl {
        (quote): '"'
        content: Content
        (quote): '"'
    }
    Content (
        (
            | {! '"' '\\' '\u{0000}'..'\u{001F}'}
            | '\\' Escape
        )*
    )
    Escape (
        | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
        | 'u' {'0'..'9' 'A'..'F' 'a'..'f'}[4]
    )
}

impl JSONStringImpl for JSONString {}
