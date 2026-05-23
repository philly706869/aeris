pub trait SyntaxShard {
    fn dependencies() -> &'static [&'static dyn SyntaxShard]
    where
        Self: Sized;
}
