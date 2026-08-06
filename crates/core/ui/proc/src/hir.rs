use proc_macro2::TokenStream;

use crate::ast::{ClusterAST, ShardAST};

#[derive(Debug)]
pub struct ClusterHIR {
    struct_shards: Vec<StructShardHIR>,
    enum_shards: Vec<EnumShardHIR>,
    lambda_shards: Vec<LambdaShardHIR>,
}

impl ClusterHIR {
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
pub struct StructShardHIR {}

#[derive(Debug)]
pub struct EnumShardHIR {}

#[derive(Debug)]
pub struct LambdaShardHIR {}
