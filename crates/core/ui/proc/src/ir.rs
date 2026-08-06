use proc_macro2::TokenStream;

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
        for shard in &ast.shards {
            match shard {
                ShardAST::Struct(shard) => struct_shards.push(todo!()),
                ShardAST::Enum(shard) => enum_shards.push(todo!()),
                ShardAST::Lambda(shard) => lambda_shards.push(todo!()),
            }
        }
        Self {
            struct_shards,
            enum_shards,
            lambda_shards,
        }
    }

    pub fn expand(&self) -> TokenStream {
        todo!()
    }
}

#[derive(Debug)]
pub struct StructShardIR {}

#[derive(Debug)]
pub struct EnumShardIR {}

#[derive(Debug)]
pub struct LambdaShardIR {}
