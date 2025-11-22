use std::collections::HashMap;

use crate::{Function, Global};

#[derive(Debug, Clone)]
pub struct Module<'f, 'g> {
    pub functions: HashMap<usize, &'f Function>,
    pub globals: HashMap<usize, &'g Global>,
}

impl<'f, 'g> Module<'f, 'g> {
    pub(crate) fn to_llvm_module(&self) -> inkwell::module::Module {
        todo!()
    }
}
