mod ast;
mod schema;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::parse2;

use crate::ast::{Cluster, Shape};

pub enum Process {
    Preprocess,
    Postprocess,
}

pub fn cluster(process: Process, input: TokenStream) -> TokenStream {
    let cluster: Cluster = parse2(input).unwrap();

    let mut expanded = TokenStream::new();
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

        let extra_impl = match process {
            Process::Preprocess => {
                let shard = schema::Shard {
                    meta: name.span().into(),
                    name: name.to_string(),
                    shape: schema::Shape::Struct(schema::Sequence::Object(Vec::new())),
                    lambdas: Vec::new(),
                };

                let hash = Literal::byte_string(&shard.meta.hash());
                let serialized = wincode::serialize(&shard).unwrap();
                let serialized = Literal::byte_string(&serialized);

                quote! {
                    impl ::aeris::ui::Preprocess for #name {
                        fn data() -> &'static ::aeris::ui::PreprocessData {
                            &::aeris::ui::PreprocessData {
                                hash: #hash,
                                serialized: #serialized,
                                dependencies: &[]
                            }
                        }
                    }
                }
            }
            Process::Postprocess => {
                quote! {}
            }
        };

        expanded.extend(quote! {
            #(#attrs)*
            #vis #keyword #name where Self: #trait_name {}
            #trait_keyword #trait_name {}
            impl ::aeris::ui::Shard for #name {}
            #extra_impl
        });
    }

    expanded.into()
}
