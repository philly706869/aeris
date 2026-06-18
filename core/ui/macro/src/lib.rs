mod ast;
mod serial;

use std::fmt::Write;

use proc_macro::{Span, TokenStream};
use quote::quote;
use sha2::{Digest, Sha256};
use syn::parse_macro_input;

use crate::ast::{Cluster, Shard};

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    let cluster = parse_macro_input!(input as Cluster);

    let hash = hash_cluster();
    let mut hash_str = String::with_capacity(64);
    for byte in &hash {
        write!(&mut hash_str, "{:02x}", byte).unwrap();
    }

    let mut expanded = proc_macro2::TokenStream::new();
    for statement in &cluster.shards {
        let stream = match statement {
            Shard::Struct(s) => {
                let attrs = &s.attrs;
                let vis = &s.vis;
                let name = &s.name;
                let trait_name = &s.trait_name;
                quote! {
                    #(#attrs)*
                    #vis struct #name where Self: #trait_name {}

                    trait #trait_name {}

                    impl ::aeris::ui::SyntaxShard for #name {}
                }
            }
            Shard::Enum(e) => {
                let attrs = &e.attrs;
                let vis = &e.vis;
                let name = &e.name;
                let trait_name = &e.trait_name;
                quote! {
                    #(#attrs)*
                    #vis enum #name where Self: #trait_name {}

                    trait #trait_name {}

                    impl ::aeris::ui::SyntaxShard for #name {}
                }
            }
            _ => quote! {},
        };
        expanded.extend(stream);
    }
    expanded.into()
}

fn hash_cluster() -> [u8; 32] {
    let span = Span::call_site();
    let file = span.file();
    let start = span.start();
    let end = span.end();
    let mut hasher = Sha256::new();
    hasher.update(&file);
    hasher.update(&start.line().to_le_bytes());
    hasher.update(&start.column().to_le_bytes());
    hasher.update(&end.line().to_le_bytes());
    hasher.update(&end.column().to_le_bytes());
    hasher.finalize().into()
}
