use crate::syntax::{Syntax, Token};

pub struct SyntaxTree {
    item: Box<dyn SyntaxTreeItem>,
    children: Vec<SyntaxTree>,
}

trait SyntaxTreeItem {}
impl<'i, T> SyntaxTreeItem for T where T: Syntax<'i> {}
impl<'t> SyntaxTreeItem for Token<'t> {}
