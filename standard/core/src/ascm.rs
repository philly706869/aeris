//! AERIS Standard Code Model

pub trait Module {}

pub trait Function {}

// pub struct Function<'f> {
//     return_type: Type<'f>,
//     parameter_types: Vec<Type<'f>>,
//     blocks: Vec<Block>,
// }

// pub struct Block {
//     instructions: Vec<Instruction>,
// }

// pub enum Instruction {}

// pub struct Type<'f> {
//     domain: Domain<'f>,
// }

// pub enum Domain<'f> {
//     External(Layout),
//     Internal(&'f Function<'f>),
// }

// pub enum Layout {}
