use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::{Arc, RwLock, RwLockReadGuard},
};

use crate::acm::{Function, Global, uid::Uid};

#[derive(Debug, Clone)]
pub struct Module {
    uid: Uid,
    functions: Arc<RwLock<HashSet<Function>>>,
    globals: Arc<RwLock<HashSet<Global>>>,
}

impl Module {
    pub fn functions(&self) -> RwLockReadGuard<'_, HashSet<Function>> {
        let functions = self.functions.read().unwrap();
        functions
    }

    pub fn add_function(&self, function: Function) -> bool {
        let mut functions = self.functions.write().unwrap();
        functions.insert(function)
    }

    pub fn remove_function(&mut self, function: &Function) -> bool {
        let mut functions = self.functions.write().unwrap();
        functions.remove(function)
    }

    pub fn globals(&self) -> RwLockReadGuard<'_, HashSet<Global>> {
        let globals = self.globals.read().unwrap();
        globals
    }

    pub fn add_global(&self, global: Global) -> bool {
        let mut globals = self.globals.write().unwrap();
        globals.insert(global)
    }

    pub fn remove_global(&self, global: &Global) -> bool {
        let mut globals = self.globals.write().unwrap();
        globals.remove(global)
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
