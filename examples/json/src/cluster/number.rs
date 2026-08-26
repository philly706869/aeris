use aeris::ui::shard;

#[shard]
pub struct JSONNumber {
    sign: Option<x!["-"]>,
    integer: x! {
        | Digit
        | One2Nine Digits
    },
    fraction: Option<JSONFraction>,
    exponent: Option<JSONExponent>,
}

#[shard]
pub struct JSONFraction {
    point: x!["."],
    digits: Digits,
}

#[shard]
pub struct JSONExponent {
    e: x![{'E' 'e'}],
    sign: Option<x![{'+' '-'}]>,
    digits: Digits,
}

#[shard]
type Digits = x! {
    | Digit+
};

#[shard]
type Digit = x! {
    | '0'
    | One2Nine
};

#[shard]
type One2Nine = x! {
    | {'1'..'9'}
};
