mod syntax;

mod proto_json {
    mod syntax {
        use aeris_ui::syntax;

        syntax! {
            pub struct JSON {
                [ws]:WS
                value:JSONValue
                [ws]:WS
            }
        }

        syntax! {
            pub enum JSONValue {
                Object ( JSONObject )
                Array ( JSONArray )
                String ( JSONString )
                Number ( JSONNumber )
                Boolean ( JSONBoolean )
                Null ( JSONNull )
            }
        }

        syntax! {
            pub struct JSONObject (

            )
        }

        syntax! {
            pub struct JSONArray
        }

        syntax! {
            pub struct JSONString { qs:'"' content:Content qe:'"' }
            Content ( ([! '\\' '\u{0000}'..'\u{001F}'] | '\\' Escape)* )
            Escape ( '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | ('u' ['0'..'9' 'A'..'F' 'a'..'f']{4}) )
        }

        syntax! {
            pub struct JSONNumber {
                sign:'-'?
                integer:(Digit | One2Nine Digits)
                fraction:()
            }
            Digits ( Digit+ )
            Digit ( '0' | One2Nine )
            One2Nine ( ['1'..'9'] )
        }

        syntax! {
            pub enum JSONBoolean {
                True ( "true" )
                False ( "false" )
            }
        }

        syntax! {
            pub struct JSONNull ( "null" )
        }

        syntax! {
            struct WS ( (" " | "\t" | "\n" | "\r")* )
        }
    }
}
