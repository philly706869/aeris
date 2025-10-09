use inkwell::{
    AddressSpace,
    context::Context,
    types::{BasicMetadataTypeEnum, BasicType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Layout {
    Word,
    Scalar { bits: u16 },
    Struct { fields: Vec<Layout>, packed: bool },
    Array { element: Box<Layout>, count: u32 },
}

impl Layout {
    pub(crate) fn map_to_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> BasicMetadataTypeEnum<'ctx> {
        todo!()
        // match self {
        //     Type::Ptr => ctx.ptr_type(AddressSpace::default()).into(),
        //     Type::Unit(bits) => ctx.custom_width_int_type(*bits as u32).into(),
        //     Type::Struct {
        //         field_types,
        //         packed,
        //     } => ctx
        //         .struct_type(
        //             &field_types
        //                 .iter()
        //                 .map(|ty| ty.map_to_llvm_type(ctx))
        //                 .collect::<Vec<_>>(),
        //             *packed,
        //         )
        //         .into(),
        //     Type::Array { item_type, size } => {
        //         item_type.map_to_llvm_type(ctx).array_type(*size).into()
        //     }
        // }
    }
}
