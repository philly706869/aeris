use aeris_ui_proc::preprocess;
use proc_macro::TokenStream;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    preprocess(input.into()).into()
}
