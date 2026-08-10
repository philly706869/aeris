mod ast;

use proc_macro2::TokenStream;
use syn::parse2;

pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_err = (!attr.is_empty()).then_some(syn::Error::new_spanned(
        attr,
        "unexpected attribute argument",
    ));

    let shard: ast::Shard = match parse2(item) {
        Ok(module) => module,
        Err(mut err) => {
            if let Some(attr_err) = attr_err {
                err.combine(attr_err);
            }
            return err.into_compile_error();
        }
    };

    TokenStream::new()
}
