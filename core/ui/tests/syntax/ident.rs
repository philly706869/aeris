use aeris_ui::syntax;

syntax! {
    pub Ident
    Ident -> name:(XIDStart XIDContinue*)
    XIDStart -> {/* TODO */}
    XIDContinue -> {/* TODO */}
}

pub struct Ident(String);
