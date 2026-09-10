use proc_macro::TokenStream;

mod shard;

///
#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    shard::shard(attr.into(), item.into()).into()
}
