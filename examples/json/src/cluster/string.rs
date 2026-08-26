use aeris::ui::shard;

#[shard]
pub struct JSONString {
    quote: x!["\""],
    content: Content,
    quote: x!["\""],
}

#[shard]
type Content = x! {
    (
        | {! '"' '\\' '\u{0000}'..'\u{001F}'}
        | '\\' Escape
    )*
};

#[shard]
type Escape = x! {
    | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
    | 'u' {'0'..'9' 'A'..'F' 'a'..'f'}[4]
};
