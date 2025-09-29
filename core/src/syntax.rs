mod context;
pub mod standard;
mod token;
mod tree;

pub use context::Context;
pub use token::{Token, TokenModifier, TokenType};

pub trait Syntax<'i> {
    type Output;

    fn mount(self, ctx: Context<'i>) -> Self::Output;
}

impl<'i, F, O> Syntax<'i> for F
where
    F: FnOnce(Context<'i>) -> O,
{
    type Output = O;

    fn mount(self, ctx: Context<'i>) -> Self::Output {
        self(ctx)
    }
}
