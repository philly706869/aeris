use std::hash::{Hash, Hasher};

use crate::acm::{Block, type_::Type, uid::Uid};

#[derive(Debug)]
pub struct Function {
    uid: Uid,
    return_type: Type,
    param_type: Vec<Type>,
    blocks: Vec<Block>,
}

impl Function {}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Function {}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}
