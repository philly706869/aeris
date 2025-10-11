pub enum Memory {
    Scalar { size: u16 },
    Array { element: Box<Memory>, count: u32 },
    Struct { fields: Vec<Memory>, packed: bool },
}

impl Memory {
    pub fn scalar(size: u16) -> Self {
        Self::Scalar { size }
    }

    pub fn array(element: Memory, count: u32) -> Self {
        Self::Array {
            element: Box::new(element),
            count,
        }
    }

    pub fn structure(fields: Vec<Memory>, packed: bool) -> Self {
        Self::Struct { fields, packed }
    }
}
