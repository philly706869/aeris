use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::FxHashMap;

use crate::ast::{ClusterAST, ShardAST};

#[derive(Debug)]
pub struct ClusterIR {
    struct_shards: Vec<StructShardIR>,
    enum_shards: Vec<EnumShardIR>,
    lambda_shards: Vec<LambdaShardIR>,
}

impl ClusterIR {
    pub fn lower(ast: &ClusterAST) -> syn::Result<Self> {
        let mut struct_shards = Vec::new();
        let mut enum_shards = Vec::new();
        let mut lambda_shards = Vec::new();
        let mut name_table = FxHashMap::default();
        let mut name_collisions = Vec::new();
        for shard in &ast.shards {
            let name_ident = match shard {
                ShardAST::Struct(shard) => {
                    let node = StructShardIR {};
                    struct_shards.push(node);
                    &shard.name
                }
                ShardAST::Enum(shard) => {
                    let node = EnumShardIR {};
                    enum_shards.push(node);
                    &shard.name
                }
                ShardAST::Lambda(shard) => {
                    let node = LambdaShardIR {};
                    lambda_shards.push(node);
                    &shard.name
                }
            };
            let name_str = name_ident.to_string();
            let collision = name_table.insert(name_str, name_ident);
            if let Some(collision) = collision {
                name_collisions.push(collision);
                name_collisions.push(name_ident);
            }
        }
        if let Some(err) = name_collisions
            .into_iter()
            .map(|name| {
                let msg = format!("duplicate shard name {}", name.to_string());
                syn::Error::new_spanned(name, msg)
            })
            .reduce(|mut acc, curr| {
                acc.combine(curr);
                acc
            })
        {
            return Err(err);
        }
        Ok(Self {
            struct_shards,
            enum_shards,
            lambda_shards,
        })
    }

    pub fn expand(&self) -> TokenStream {
        quote! {}
    }
}

#[derive(Debug)]
pub struct StructShardIR {}

#[derive(Debug)]
pub struct EnumShardIR {}

#[derive(Debug)]
pub struct LambdaShardIR {}
