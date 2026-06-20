mod ast;
mod schema;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::parse2;

use crate::ast::{Cluster, Sequence, Shape, Shard};

#[derive(Clone, Copy)]
pub enum Process {
    Preprocess,
    Postprocess,
}

pub fn cluster(process: Process, input: TokenStream) -> TokenStream {
    match parse2::<Cluster>(input) {
        Ok(cluster) => {
            let expandeds = cluster.shards.iter().map(|shard| {
                let preprocess_impl = match process {
                    Process::Preprocess => Some(gen_preprocess(shard)),
                    Process::Postprocess => None,
                };
                let base = gen_shard(shard);
                quote! { #base #preprocess_impl }
            });
            quote! { #(#expandeds)* }
        }
        Err(err) => err.into_compile_error(),
    }
}

fn gen_shard(shard: &Shard) -> TokenStream {
    let attrs = &shard.attrs;
    let vis = &shard.vis;
    let name = &shard.name;
    let trait_name = &shard.trait_name;
    let keyword;
    let def;
    match &shard.shape {
        Shape::Struct(sequence) => {
            keyword = quote! { struct };
            def = match sequence {
                Sequence::Object(entries) => {
                    quote! { where Self: #trait_name {} }
                }
                Sequence::Tuple(entries) => {
                    quote! { () where Self: #trait_name; }
                }
            };
        }
        Shape::Enum(variants) => {
            keyword = quote! { enum };
            let variants = variants.iter().map(|(name, sequence)| match sequence {
                Sequence::Object(entries) => {
                    quote! { #name {} }
                }
                Sequence::Tuple(entries) => {
                    quote! { #name () }
                }
            });
            def = quote! { where Self: #trait_name { #(#variants),* } };
        }
    };
    quote! {
        #(#attrs)*
        #vis #keyword #name #def
        trait #trait_name {}
        impl ::aeris::ui::Shard for #name {}
    }
}

fn gen_preprocess(shard: &Shard) -> TokenStream {
    let name = &shard.name;
    let shard = schema::Shard {
        meta: name.span().into(),
        name: name.to_string(),
        shape: schema::Shape::Struct(schema::Sequence::Object(Vec::new())),
        lambdas: Vec::new(),
    };

    let hash = Literal::byte_string(&shard.meta.hash());
    let serialized = wincode::serialize(&shard).unwrap();
    let serialized = Literal::byte_string(&serialized);

    let dependencies: Vec<syn::Ident> = Vec::new();

    quote! {
        const _: () = {
            use ::aeris::ui::preproc::{Data, Shard};
            fn d<S>() -> &'static Data where S: Shard { S::data() }
            static DATA: Data = Data {
                hash: #hash,
                serialized: #serialized,
                dependencies: &[#(d::<#dependencies>()),*],
            };
            impl Shard for #name { fn data() -> &'static Data { &DATA } }
        };
    }
}
