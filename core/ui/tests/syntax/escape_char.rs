use aeris_ui::syntax;

syntax! {
    pub EscapeChar
    EscapeChar -> '\\' (StandardEscape | HexEscape | UnicodeEscape | UnicodeVariableEscape)
    StandardEscape -> {'\\' '"' '\'' 'n' 't' 'r' 'b' 'f'}
    HexEscape -> 'x' HexDigit HexDigit
    UnicodeEscape -> 'u' HexDigit HexDigit HexDigit HexDigit
    UnicodeVariableEscape -> 'u' '{' HexDigit[0,6] '}'
    HexDigit -> {'0'..'9' 'a'..'f' 'A'..'F'}
}
