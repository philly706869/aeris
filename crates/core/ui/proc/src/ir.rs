use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::FxHashSet;

use crate::ast::{ClusterAST, ShardAST};

#[derive(Debug)]
pub struct ClusterIR {
    struct_shards: Vec<StructShardIR>,
    enum_shards: Vec<EnumShardIR>,
    lambda_shards: Vec<LambdaShardIR>,
}

impl ClusterIR {
    pub fn lower(ast: &ClusterAST) -> Self {
        let mut struct_shards = Vec::new();
        let mut enum_shards = Vec::new();
        let mut lambda_shards = Vec::new();
        let mut names = FxHashSet::default();
        for shard in &ast.shards {
            match shard {
                ShardAST::Struct(shard) => {
                    let node = StructShardIR {};
                    struct_shards.push(node);
                    let name = shard.name.to_string();
                    names.insert(name);
                }
                ShardAST::Enum(shard) => {
                    let node = EnumShardIR {};
                    enum_shards.push(node);
                    let name = shard.name.to_string();
                    names.insert(name);
                }
                ShardAST::Lambda(shard) => {
                    let node = LambdaShardIR {};
                    lambda_shards.push(node);
                    let name = shard.name.to_string();
                    names.insert(name);
                }
            }
        }
        Self {
            struct_shards,
            enum_shards,
            lambda_shards,
        }
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
