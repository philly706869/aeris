mod context;
mod token;
mod tree;

pub use context::Context;
pub use token::{Token, TokenModifier, TokenType};

pub trait Syntax<'i> {
    type Output;

    fn mount(ctx: Context<'i>) -> Self::Output
    where
        Self: Sized;
}
