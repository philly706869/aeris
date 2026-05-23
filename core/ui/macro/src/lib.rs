mod ast;

use proc_macro::TokenStream;

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    TokenStream::new()
}
