use aeris_ui_proc::postprocess;
use proc_macro::TokenStream;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    postprocess(input.into()).into()
}
