use aeris::ui::shard;

#[shard]
pub mod JSONString {
    #[derive(Debug)]
    struct Shard {
        quote: x!["\""],
        content: Content,
        quote: x!["\""],
    }
}

#[shard]
mod Content {
    x! {
        (
            | {! '"' '\\' '\u{0000}'..'\u{001F}'}
            | '\\' Escape
        )*
    }
}

#[shard]
mod Escape {
    x! {
        | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
        | 'u' {'0'..'9' 'A'..'F' 'a'..'f'}[4]
    }
}
