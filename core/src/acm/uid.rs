use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Uid(Uuid);

impl Uid {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Uid {
    fn default() -> Self {
        Self::new()
    }
}
