mod span;

pub use span::Span;

pub struct Context<'i> {
    full_input: &'i str,
    input: &'i str,
}

impl<'i> Context<'i> {
    pub(crate) fn new(full_input: &'i str, input: &'i str) -> Self {
        Self { full_input, input }
    }

    pub fn full_input(&self) -> &str {
        self.full_input
    }

    pub fn input(&self) -> &str {
        self.input
    }

    pub fn span(&self) -> Span {
        todo!()
    }
}
