mod ast;

use proc_macro::TokenStream;

#[proc_macro]
pub fn syntax(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
