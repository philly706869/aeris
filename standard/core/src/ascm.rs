//! AERIS Standard Code Model

use std::collections::HashMap;

pub struct Module {
    pub table: HashMap<String, Item>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

pub enum Item {
    Module(Module),
    Function(Function),
    Global(Global),
}

pub struct Function {
    return_type: (),
    params: Vec<()>,
    blocks: Option<Vec<Block>>,
}

pub struct Block {
    instructions: Vec<Instruction>,
}

pub enum Instruction {}

pub struct Global {}

pub trait MemoryLayout {}

pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    F128,
    Tuple(Tuple),
    Class(Class),
}

pub struct Tuple {
    elements: Vec<Type>,
}

pub struct Array {
    element_type: Type,
    count: usize,
}

pub struct Class {
    parents: Vec<Type>,
    fields: Vec<Type>,
}
