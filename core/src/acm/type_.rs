#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Ptr,
    Void,
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    F128,
    Struct(Vec<Type>),
    Array(Box<Type>, usize),
}
