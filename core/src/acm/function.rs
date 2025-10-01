use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
    sync::Arc,
};

use crate::acm::uid::Uid;

#[derive(Clone)]
pub struct Function {
    uid: Uid,
    data: Rc<RefCell<FunctionData>>,
}

impl Function {}

struct FunctionData {}

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
