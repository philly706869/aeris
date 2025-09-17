mod token;

pub use token::{/*Token, */ TokenModifier, TokenType};

pub trait Syntax {
    type Output;
    type Error;

    fn mount(&mut self, input: &str);

    fn unmount(&mut self);

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error>;
}
