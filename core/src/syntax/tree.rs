// use crate::syntax::{Syntax, Token};

// pub trait SyntaxTree {}

// pub struct SyntaxTreeBranch(pub Vec<Box<dyn SyntaxTree>>);

// impl SyntaxTree for SyntaxTreeBranch {}

// pub struct SyntaxTreeLeaf<'i, Output> {
//     syntax: Box<dyn Syntax<'i, Output = Output>>,
//     token: Token<'i>,
// }

// impl<'i, Output> SyntaxTree for SyntaxTreeLeaf<'i, Output> {}
