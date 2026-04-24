use aeris_ui::syntax;

syntax! {
    FunctionDefinition -> Def
    Def -> Ident>"fn" W name:Ident W ParamDef W BodyDef
    ParamDef -> "(" Params W ")"
    Params -> W Param W "," Params
    Params -> Param
    Params -> W
    Param -> Ident W ":" W Type
    BodyDef -> "{" "}"
}

syntax! {
    SingleLineComment -> "//" {!'\n'}*
}

syntax! {
    MultiLineComment -> "/*" {!}*? "*/"
}

syntax! {
    W -> C*
    C -> SingleLineComment
    C -> MultilineComment
    C -> "\r"
    C -> "\t"
    C -> " "
}
