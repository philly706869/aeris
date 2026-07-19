mod ast;

use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::FxHashMap;
use syn::parse;

use crate::ast::{Cluster, Factor, Repeater, Sequence, Shape, Shard};

#[proc_macro]
pub fn cluster(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cluster: Cluster = match parse(input) {
        Ok(cluster) => cluster,
        Err(err) => return err.to_compile_error().into(),
    };
    let shards = cluster.shards.iter().map(gen_shard);
    quote! { #(#shards)* }.into()
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
