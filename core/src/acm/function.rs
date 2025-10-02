use std::{
    hash::{Hash, Hasher},
    sync::{Arc, RwLock, RwLockReadGuard},
};

use indexmap::IndexSet;

use crate::acm::{Block, uid::Uid};

#[derive(Debug, Clone)]
pub struct Function {
    uid: Uid,
    blocks: Arc<RwLock<IndexSet<Block>>>,
}

impl Function {
    pub fn blocks(&self) -> RwLockReadGuard<'_, IndexSet<Block>> {
        let blocks = self.blocks.read().unwrap();
        blocks
    }

    pub fn add_block(&self, block: Block) -> bool {
        let mut blocks = self.blocks.write().unwrap();
        blocks.insert(block)
    }

    pub fn remove_block(&self, block: &Block) -> bool {
        let mut blocks = self.blocks.write().unwrap();
        blocks.shift_remove(block)
    }
}

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
