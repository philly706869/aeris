use aeris_ui::syntax;

syntax! {
    pub MultiLineComment
    MultiLineComment -> "/*" Body
    Body -> {!'*'} Body
    Body -> '*' AfterAsterisk
    AfterAsterisk -> '/'
    AfterAsterisk -> '*' AfterAsterisk
    AfterAsterisk -> {!'*' '/'} Body
}
