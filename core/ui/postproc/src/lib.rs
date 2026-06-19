use aeris_ui_proc as proc;
use proc_macro::TokenStream;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    proc::cluster(proc::Process::Postprocess, input.into()).into()
}
