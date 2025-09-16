use std::collections::VecDeque;

pub struct SyntaxSource<'st, 't> {
    syntax_tree: &'st SyntaxTree,
    syntax_tokens: Vec<Token<'t>>,
}

impl<'st, 't> SyntaxSource<'st, 't> {
    pub fn new(syntax_tree: &'st SyntaxTree) -> Self {
        Self {
            syntax_tree,
            syntax_tokens: Vec::new(),
        }
    }

    pub fn update<'nt>(mut self, input: &'nt str) -> SyntaxSource<'st, 'nt> {
        let tree = self.syntax_tree;
        let mut parse_queue = VecDeque::<Box<dyn Fn()>>::new();

        todo!()
    }
}

pub trait Syntax<Context, Return> {
    fn parse(
        &self,
        ctx: SyntaxContext<Context>,
    ) -> Result<(usize,), Option<Box<dyn Syntax<Context, Return>>>>;
}

pub enum SyntaxAction<Context> {
    Return(Context),
}

pub struct SyntaxContext<Context> {
    context: Context,
}

pub struct Token<'t> {
    text: &'t str,
    type_: TokenType,
    modifier: TokenModifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TokenType {
    #[default]
    Unknown,
    Keyword,
    Type,
    Identifier,
    Literal,
}

#[derive(Debug)]
pub struct TokenModifier {}
