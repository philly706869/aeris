mod block;
mod function;
mod global;
mod layout;
mod module;
mod uid;

pub use block::Block;
pub use function::Function;
pub use global::Global;
pub use module::Module;

use std::collections::HashMap;

pub struct AERISCodeModel {
    items: HashMap<usize, Item>,
}

impl AERISCodeModel {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

enum Item {
    Function,
}
