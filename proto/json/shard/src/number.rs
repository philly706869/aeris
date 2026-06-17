use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct JSONNumber
    trait JSONNumberImpl {
        sign: '-'?
        integer: ( Digit | One2Nine Digits )
        fraction: JSONFraction?
        exponent: JSONExponent?
    }

    #[derive(Debug)]
    pub struct JSONFraction
    trait JSONFractionImpl {
        point: '.'
        digits: Digits
    }

    #[derive(Debug)]
    pub struct JSONExponent
    trait JSONExponentImpl {
        e: {'E' 'e'}
        sign: JSONSign?
        digits: Digits
    }

    #[derive(Debug)]
    pub enum JSONSign
    trait JSONSignImpl {
        Plus ( '+' )
        Minus ( '-' )
    }

    Digits ( Digit+ )
    Digit ( '0' | One2Nine )
    One2Nine ( {'1'..'9'} )
}

impl JSONNumberImpl for JSONNumber {}
impl JSONFractionImpl for JSONFraction {}
impl JSONExponentImpl for JSONExponent {}
impl JSONSignImpl for JSONSign {}
