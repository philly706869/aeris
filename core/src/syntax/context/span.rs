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
