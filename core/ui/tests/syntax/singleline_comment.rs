use aeris_ui::syntax;

syntax! {
    pub SingleLineComment
    SingleLineComment -> "//" {!'\n'}*
}
