mod ast;
mod schema;

use std::{
    fmt::Write,
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

use proc_macro::{Span, TokenStream};
use quote::quote;
use sha2::{Digest, Sha256};
use syn::parse_macro_input;

use crate::ast::{Cluster, Shard};

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    let cluster = parse_macro_input!(input as Cluster);

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

    let span = Span::call_site();
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
    let hash_bytes: [u8; 32] = hasher.finalize().into();
    let mut hash = String::with_capacity(64);
    for byte in &hash_bytes {
        write!(&mut hash, "{:02x}", byte).unwrap();
    }

    let cluster = schema::Cluster {
        file,
        start_line: start_line as u64,
        start_column: start_column as u64,
        end_line: end_line as u64,
        end_column: end_column as u64,
        shards: Vec::new(),
    };

    let bytes = wincode::serialize(&cluster).unwrap();

    expanded.into()
}
