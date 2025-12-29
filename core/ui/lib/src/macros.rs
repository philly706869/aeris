#[macro_export]
macro_rules! re {
    ($context:ident $($code: tt)*) => {{
        |$context: ContextType| { ($code)* }
    }};
}
