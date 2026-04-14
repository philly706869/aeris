use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syntax/comment.pest"]
#[grammar = "syntax/identifier.pest"]
#[grammar = "syntax/keyword.pest"]
pub struct AERISParser;
