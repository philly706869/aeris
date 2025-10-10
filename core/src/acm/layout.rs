use inkwell::{
    context::Context,
    types::{BasicType, BasicTypeEnum},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Layout {
    Scalar { bits: u16 },
    Struct { fields: Vec<Layout>, packed: bool },
    Array { element: Box<Layout>, count: u32 },
}

impl Layout {
    pub(crate) fn to_llvm_basic_type_enum<'ctx>(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Self::Scalar { bits } => ctx.custom_width_int_type(*bits as u32).into(),
            Self::Struct { fields, packed } => ctx
                .struct_type(
                    &fields
                        .iter()
                        .map(|layout| layout.to_llvm_basic_type_enum(ctx))
                        .collect::<Vec<_>>(),
                    *packed,
                )
                .into(),
            Self::Array { element, count } => element
                .to_llvm_basic_type_enum(ctx)
                .array_type(*count)
                .into(),
        }
    }
}
