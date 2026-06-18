use wincode::{SchemaRead, SchemaWrite};

#[derive(SchemaRead, SchemaWrite)]
pub struct Cluster {
    file: String,
    start_line: u64,
    start_column: u64,
    end_line: u64,
    end_column: u64,
    shards: Vec<Shard>,
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Shard {
    name: String,
    shape: Shape,
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Shape {
    Struct(Sequence),
    Enum(Vec<(String, Sequence)>),
    Lambda(Entry),
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Sequence {
    Object(Vec<(String, Entry)>),
    Tuple(Vec<Entry>),
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Entry {
    factor: Factor,
    quantifier: Option<Quantifier>,
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Factor {
    Literal(String),
    Set(Set),
    Term(Vec<Vec<Entry>>),
    Shard(String),
    Extern(String),
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Quantifier {
    repeater: Repeater,
    lazy: bool,
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Repeater {
    Plus,
    Star,
    Option,
    Val(u32),
    Min(u32),
    Max(u32),
    Range(u32, u32),
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Set {
    Positive(Vec<SetEntry>),
    Negative(Vec<SetEntry>),
}

#[derive(SchemaRead, SchemaWrite)]
pub enum SetEntry {
    Single(char),
    Range(char, char),
}
