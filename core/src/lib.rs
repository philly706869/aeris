pub mod ls;
pub mod reactive;

pub struct AERIS {}

impl AERIS {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum SyntaxTree {
    Or(OrTree),
    And(AndTree),
    Leaf(Box<dyn SyntaxComponent>),
}

pub struct OrTree {
    items: Vec<SyntaxTree>,
}

pub struct AndTree {
    items: Vec<SyntaxTree>,
}

pub trait SyntaxComponent {
    fn parse(&mut self, ctx: &mut SyntaxParseContext) -> Result<(), ()>;
}

pub struct SyntaxParseContext {}

pub struct SyntaxToken<'t> {
    text: &'t str,
    type_: SyntaxTokenType,
    modifier: SyntaxTokenModifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SyntaxTokenType {
    #[default]
    Unknown,
    Keyword,
    Type,
    Identifier,
    Literal,
}

#[derive(Debug)]
pub struct SyntaxTokenModifier {}
