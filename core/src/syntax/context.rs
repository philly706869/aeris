pub struct Context<'i> {
    input: &'i str,
}

impl<'i> Context<'i> {
    pub(crate) fn new(input: &'i str) -> Self {
        Self { input }
    }

    pub fn input(&self) -> Input<'_> {
        Input(self.input)
    }

    pub fn span(&self) -> Span {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Input<'i>(&'i str);

impl<'i> Input<'i> {}

pub struct Span {}

impl Span {
    pub fn capture(&self) -> &str {
        todo!()
    }

    pub fn attach_token(&self) {
        todo!()
    }
}

impl Iterator for Span {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
