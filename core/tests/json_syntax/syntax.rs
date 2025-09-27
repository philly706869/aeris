use aeris::syntax::{Context, Syntax};

pub struct JSONSyntax;

impl<'i> Syntax<'i> for JSONSyntax {
    type Output = ();

    fn parse(&self, ctx: Context<'i>) -> Self::Output {
        todo!()
    }
}
