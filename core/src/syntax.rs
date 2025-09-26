mod context;
mod token;

pub use context::Context;
pub use token::{Token, TokenModifier, TokenType};

pub trait Syntax<'i> {
    type Output;

    fn parse(&self, ctx: Context<'i>) -> Self::Output;
}
