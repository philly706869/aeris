mod ast;
mod hir;

use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::FxHashMap;
use syn::parse;

use crate::ast::{ClusterAST, FactorAST, RepeaterAST, SequenceAST, ShapeAST, ShardAST};

#[proc_macro]
pub fn cluster(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cluster: ClusterAST = match parse(input) {
        Ok(cluster) => cluster,
        Err(err) => return err.to_compile_error().into(),
    };
    let shards = cluster.shards.iter().map(gen_shard);
    quote! { #(#shards)* }.into()
}

fn gen_shard(shard: &ShardAST) -> TokenStream {
    let attrs = &shard.attrs;
    let vis = &shard.vis;
    let name = &shard.name;
    let trait_name = &shard.trait_name;
    let keyword;
    let def;
    match &shard.shape {
        ShapeAST::Struct(sequence) => {
            keyword = quote! { struct };
            def = match sequence {
                SequenceAST::Object(entries) => {
                    let mut grouped_entries: FxHashMap<_, Vec<_>> = FxHashMap::default();
                    for (name, entry) in entries.iter() {
                        let type_ = match &entry.factor {
                            FactorAST::Shard(ident) => quote! { #ident },
                            FactorAST::LitStr(str) => quote! { () },
                            FactorAST::LitChar(char) => quote! { () },
                            FactorAST::Set(set) => quote! { () },
                            FactorAST::Term(term) => quote! { () },
                        };
                        let type_ = match &entry.quantifier {
                            Some(quantifier) => match &quantifier.repeater {
                                RepeaterAST::Plus => quote! { Vec<#type_> },
                                RepeaterAST::Star => quote! { Vec<#type_> },
                                RepeaterAST::Option => quote! { Option<#type_> },
                                RepeaterAST::Val(val) => quote! { [#type_; #val] },
                                RepeaterAST::Min(_) => quote! { Vec<#type_> },
                                RepeaterAST::Max(_) => quote! { Vec<#type_> },
                                RepeaterAST::Range(_, _) => quote! { Vec<#type_> },
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
                SequenceAST::Tuple(entries) => {
                    quote! { () where Self: #trait_name; }
                }
            };
        }
        ShapeAST::Enum(variants) => {
            keyword = quote! { enum };
            let variants = variants.iter().map(|(name, sequence)| match sequence {
                SequenceAST::Object(entries) => {
                    quote! { #name {} }
                }
                SequenceAST::Tuple(entries) => {
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
