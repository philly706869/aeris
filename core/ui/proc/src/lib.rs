mod ast;
mod schema;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::parse2;

use crate::ast::{Cluster, Enum, Sequence, Shard, Struct};

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
                let base = match shard {
                    Shard::Struct(struct_) => gen_struct(struct_),
                    Shard::Enum(enum_) => gen_enum(enum_),
                };
                quote! { #base #preprocess_impl }
            });
            quote! { #(#expandeds)* }
        }
        Err(err) => err.into_compile_error(),
    }
}

fn gen_struct(shard: &Struct) -> TokenStream {
    let attrs = &shard.attrs;
    let vis = &shard.vis;
    let name = &shard.name;
    let keyword = &shard.keyword;
    let trait_keyword = &shard.trait_keyword;
    let trait_name = &shard.trait_name;
    let sequence = &shard.sequence;
    let body = match sequence {
        Sequence::Object(_) => quote! { where Self: #trait_name {} },
        Sequence::Tuple(_) => quote! { () where Self: #trait_name; },
    };
    quote! {
        #(#attrs)*
        #vis struct #name #body
        #trait_keyword #trait_name {}
        impl ::aeris::ui::Shard for #name {}
    }
}

fn gen_enum(shard: &Enum) -> TokenStream {
    let attrs = &shard.attrs;
    let vis = &shard.vis;
    let name = &shard.name;
    let keyword = &shard.keyword;
    let trait_keyword = &shard.trait_keyword;
    let trait_name = &shard.trait_name;
    let variants = &shard.variants;
    let variants = variants.iter().map(|(name, sequence)| {
        let body = match sequence {
            Sequence::Object(_) => quote! { {} },
            Sequence::Tuple(_) => quote! { () },
        };
        quote! { #name #body }
    });
    let body = quote! { where Self: #trait_name { #(#variants),* } };
    quote! {
        #(#attrs)*
        #vis #keyword #name #body
        #trait_keyword #trait_name {}
        impl ::aeris::ui::Shard for #name {}
    }
}

fn gen_preprocess(shard: &Shard) -> TokenStream {
    let name = match shard {
        Shard::Struct(struct_) => &struct_.name,
        Shard::Enum(enum_) => &enum_.name,
    };
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
