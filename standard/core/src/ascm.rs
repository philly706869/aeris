//! AERIS Standard Code Model

use std::collections::HashMap;

pub struct Function {
    return_type: Option<Type>,
    parameter_type: Type,
    blocks: HashMap<usize, Block>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    S1,
    S8,
    S16,
    S32,
    S64,
    S128,
    Ptr,
}

pub struct Block {
    instructions: Vec<Instruction>,
}

impl Block {}

pub struct Instruction {
    data: Box<dyn inst::Instruction>,
}

mod inst {
    use super::Type;

    pub trait Instruction {
        fn ssa_type(&self) -> Option<Type>;
    }

    pub struct Alloca {
        ttype: Type,
    }

    impl Instruction for Alloca {
        fn ssa_type(&self) -> Option<Type> {
            Some(Type::Ptr)
        }
    }

    pub struct Store {
        ttype: Type,
    }

    impl Instruction for Store {
        fn ssa_type(&self) -> Option<Type> {
            None
        }
    }

    pub struct Load {
        ttype: Type,
    }

    impl Instruction for Load {
        fn ssa_type(&self) -> Option<Type> {
            Some(self.ttype)
        }
    }

    pub struct Add {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
