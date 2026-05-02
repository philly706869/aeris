use aeris_ui::syntax;

syntax! {
    pub W
    W -> C+
    C -> SingleLineComment
    C -> MultilineComment
    C -> '\t' | '\n' | '\r' | ' '
}
