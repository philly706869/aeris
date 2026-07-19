mod ast;
mod hir;
mod mir;
mod schema;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use rustc_hash::{FxHashMap, FxHashSet};
use syn::{Ident, parse2};

use crate::ast::{Cluster, Factor, Repeater, Sequence, Shape, Shard};

pub fn preprocess(input: TokenStream) -> TokenStream {
    let cluster = match parse2(input) {
        Ok(cluster) => cluster,
        Err(err) => return err.to_compile_error(),
    };
    let base = gen_cluster(&cluster);
    let impls = gen_preprocess(&cluster);
    quote! { #base #impls }
}

pub fn postprocess(input: TokenStream) -> TokenStream {
    let cluster = match parse2(input) {
        Ok(cluster) => cluster,
        Err(err) => return err.to_compile_error(),
    };
    gen_cluster(&cluster)
}

fn gen_cluster(cluster: &Cluster) -> TokenStream {
    let shards = cluster.shards.iter().map(gen_shard);
    quote! { #(#shards)* }
}

fn gen_preprocess(cluster: &Cluster) -> TokenStream {
    let names: FxHashSet<String> = cluster
        .lambdas
        .iter()
        .map(|labmda| labmda.name.to_string())
        .collect();
    let impls = cluster
        .shards
        .iter()
        .map(|shard| gen_preprocess_impl(shard, &names));
    quote! { #(#impls)* }
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
                    let mut grouped_entries: FxHashMap<_, Vec<_>> = FxHashMap::default();
                    for (name, entry) in entries.iter() {
                        let type_ = match &entry.factor {
                            Factor::Shard(ident) => quote! { #ident },
                            Factor::LitStr(str) => quote! { () },
                            Factor::LitChar(char) => quote! { () },
                            Factor::Set(set) => quote! { () },
                            Factor::Term(term) => quote! { () },
                        };
                        let type_ = match &entry.quantifier {
                            Some(quantifier) => match &quantifier.repeater {
                                Repeater::Plus => quote! { Vec<#type_> },
                                Repeater::Star => quote! { Vec<#type_> },
                                Repeater::Option => quote! { Option<#type_> },
                                Repeater::Val(val) => quote! { [#type_; #val] },
                                Repeater::Min(_) => quote! { Vec<#type_> },
                                Repeater::Max(_) => quote! { Vec<#type_> },
                                Repeater::Range(_, _) => quote! { Vec<#type_> },
                            },
                            None => type_,
                        };
                        grouped_entries.entry(name).or_default().push(type_);
                    }
                    let entries = grouped_entries.iter().map(|(name, types)| {
                        quote! { #name: (#(#types),*) }
                    });
                    quote! { where Self: #trait_name { #(#entries),* } }
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

fn gen_preprocess_impl(shard: &Shard, names: &FxHashSet<String>) -> TokenStream {
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

    let mut deps: Vec<Ident> = Vec::new();

    quote! {
        const _: () = {
            use ::aeris::ui::preproc::{Meta, Shard};
            impl Shard for #name {
                fn meta() -> &'static dyn Meta {
                    &META
                }
            }
            struct META;
            impl Meta for META {
                fn hash(&self) -> &'static [u8] {
                    #hash
                }
                fn data(&self) -> &'static [u8] {
                    #serialized
                }
                fn deps(&self) -> &'static [&'static dyn Meta] {
                    &[#(meta::<#deps>()),*]
                }
            }
            fn meta<S>() -> &'static dyn Meta
            where
                S: Shard,
            {
                S::meta()
            }
        };
    }
}
