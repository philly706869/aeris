use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::acm::uid::Uid;

#[derive(Debug, Clone)]
pub struct Block {
    uid: Uid,
}

impl Block {}

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
