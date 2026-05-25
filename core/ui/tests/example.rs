mod syntax;

mod proto_json {
    use aeris_ui::syntax;

    syntax! {
        struct JSONWhiteSpace -> (" " | "\t" | "\n" | "\r")*;
    }

    // use self::JSONWhiteSpace as S

    syntax! {
        pub enum JSONNumber {
            Int -> int:Digit+;
            Float -> int:Digit+ p:"." frac:Digit+;
        }
        Digit -> ['0'..'9'];
    }
}
