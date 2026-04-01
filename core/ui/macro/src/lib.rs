mod cfg;

use proc_macro::TokenStream;

#[proc_macro]
pub fn cfg(token_stream: TokenStream) -> TokenStream {
    cfg::cfg(token_stream.into()).into()
}
