use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::{Arc, RwLock},
};

use crate::acm::{Function, Global, uid::Uid};

pub struct Module {
    uid: Uid,
    functions: Arc<RwLock<HashSet<Function>>>,
    globals: Arc<RwLock<HashSet<Global>>>,
}

impl Module {
    pub fn functions(&self) -> Arc<RwLock<HashSet<Function>>> {
        self.functions.clone()
    }

    pub fn add_function(&self, function: Function) -> bool {
        let mut function_set = self.functions.write().unwrap();
        function_set.insert(function)
    }

    pub fn remove_function(&mut self, function: &Function) -> bool {
        let mut function_set = self.functions.write().unwrap();
        function_set.remove(function)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Module {}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}
