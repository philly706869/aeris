use aeris::ui::shard;

#[shard]
pub mod JSONNumber {
    #[derive(Debug)]
    struct Shard {
        sign: Option<x!["-"]>,
        integer: x! {
            | Digit
            | One2Nine Digits
        },
        fraction: Option<JSONFraction>,
        exponent: Option<JSONExponent>,
    }
}

#[shard]
pub mod JSONFraction {
    #[derive(Debug)]
    struct Shard {
        point: x!["."],
        digits: Digits,
    }
}

#[shard]
pub mod JSONExponent {
    #[derive(Debug)]
    struct Shard {
        e: x![{'E' 'e'}],
        sign: Option<JSONSign>,
        digits: Digits,
    }
}

#[shard]
pub mod JSONSign {
    #[derive(Debug)]
    enum Shard {
        Plus(x!["+"]),
        Minus(x!["-"]),
    }
}

#[shard]
mod Digits {
    Shard! {
        | Digit+
    }
}

#[shard]
mod Digit {
    Shard! {
        | '0'
        | One2Nine
    }
}

#[shard]
mod One2Nine {
    Shard! {
        | {'1'..'9'}
    }
}
