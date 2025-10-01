use std::hash::{Hash, Hasher};

use crate::acm::uid::Uid;

pub struct Global {
    uid: Uid,
}

impl Global {}

impl PartialEq for Global {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Global {}

impl Hash for Global {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}
