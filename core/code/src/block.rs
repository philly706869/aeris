use std::hash::{Hash, Hasher};

use crate::uid::Uid;

#[derive(Debug, Clone)]
pub struct Block {
    uid: Uid,
    pub instructions: Vec<()>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            uid: Uid::new(),
            instructions: Vec::new(),
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Block {}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}
