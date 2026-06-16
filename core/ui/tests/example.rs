mod syntax;

mod proto_json {
    mod syntax {
        // use aeris_ui::syntax;

        syntax! {
            #[derive(Debug)]
            pub struct JSON {
                (ws): WS
                value: JSONValue
                (ws): WS
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
            pub struct JSONObject {
                (brace): '{'
                content: JSONObjectContent
                (brace): '}'
            }

            pub enum JSONObjectContent {
                None ( WS )
                Some {
                    first: JSONObjectEntry
                    rest: JSONObjectRestEntry*
                }
            }

            pub struct JSONObjectRestEntry {
                rest: ','
                entry: JSONObjectEntry
            }

            pub struct JSONObjectEntry {
                (ws): WS
                name: JSONString
                (ws): WS
                colon: ':'
                (ws): WS
                value: JSONValue
                (ws): WS
            }
        }

        syntax! {
            pub struct JSONArray {
                (bracket): '['
                content: JSONArrayContent
                (bracket): ']'
            }

            pub enum JSONArrayContent {
                None ( WS )
                Some {
                    first: JSONArrayEntry
                    rest: JSONArrayRestEntry*
                }
            }

            pub struct JSONArrayRestEntry {
                rest: ','
                entry: JSONArrayEntry
            }

            pub struct JSONArrayEntry {
                (ws): WS
                value: JSONValue
                (ws): WS
            }
        }

        syntax! {
            pub struct JSONString {
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

        syntax! {
            pub struct JSONNumber {
                sign: '-'?
                integer: (Digit | One2Nine Digits)
                fraction: JSONFraction?
                exponent: JSONExponent?
            }

            pub struct JSONFraction {
                point: '.'
                digits: Digits
            }

            pub struct JSONExponent {
                e: {'E' 'e'}
                sign: JSONSign?
                digits: Digits
            }

            pub enum JSONSign {
                Plus ( '+' )
                Minus ( '-' )
            }

            Digits ( Digit+ )
            Digit ( '0' | One2Nine )
            One2Nine ( {'1'..'9'} )
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
            pub struct WS ( {' ' '\t' '\n' '\r'}* )
        }
    }
}
