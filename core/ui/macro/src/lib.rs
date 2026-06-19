mod ast;
mod schema;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use syn::parse_macro_input;

use crate::ast::{Cluster, Shape};

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    let cluster = parse_macro_input!(input as Cluster);

    let mut expanded = TokenStream2::new();
    for shard in &cluster.shards {
        let attrs = &shard.attrs;
        let vis = &shard.vis;
        let name = &shard.name;
        let trait_keyword = &shard.trait_keyword;
        let trait_name = &shard.trait_name;

        let keyword = match &shard.shape {
            Shape::Struct { keyword, sequence } => quote! { #keyword },
            Shape::Enum { keyword, variants } => quote! { #keyword },
        };

        let preprocess_impl = if cfg!(feature = "preprocess") {
            let span = name.span().unwrap();
            let file = span.file();
            let start = span.start();
            let start_line = start.line();
            let start_column = start.column();
            let end = span.end();
            let end_line = end.line();
            let end_column = end.column();

            let mut hasher = Sha256::new();
            hasher.update(&file);
            hasher.update(start_line.to_le_bytes());
            hasher.update(start_column.to_le_bytes());
            hasher.update(end_line.to_le_bytes());
            hasher.update(end_column.to_le_bytes());
            let hash: [u8; 32] = hasher.finalize().into();

            let shard = schema::Shard {
                file,
                start_line: start_line as u64,
                start_column: start_column as u64,
                end_line: end_line as u64,
                end_column: end_column as u64,
                name: name.to_string(),
                shape: schema::Shape::Struct(schema::Sequence::Object(Vec::new())),
                lambdas: Vec::new(),
            };

            let bytes = wincode::serialize(&shard).unwrap();
            let hash = Literal::byte_string(&hash);
            let data = Literal::byte_string(&bytes);
            Some(quote! {
                impl ::aeris::ui::Preprocess for #name {
                    fn hash() -> &'static [u8] { #hash }
                    fn data() -> &'static [u8] { #data }
                }
            })
        } else {
            None
        };

        expanded.extend(quote! {
            #(#attrs)*
            #vis #keyword #name where Self: #trait_name {}
            #trait_keyword #trait_name {}
            impl ::aeris::ui::Shard for #name {}
            #preprocess_impl
        });
    }

    expanded.into()
}
