use proc_macro::Span;
use sha2::{Digest, Sha256};
use wincode::{SchemaRead, SchemaWrite};

#[derive(SchemaRead, SchemaWrite)]
pub struct Shard {
    pub meta: Meta,
    pub name: String,
    pub shape: Shape,
    pub lambdas: Vec<Lambda>,
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Meta {
    pub file: String,
    pub start_line: u64,
    pub start_column: u64,
    pub end_line: u64,
    pub end_column: u64,
}

impl Meta {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.file);
        hasher.update(self.start_line.to_le_bytes());
        hasher.update(self.start_column.to_le_bytes());
        hasher.update(self.end_line.to_le_bytes());
        hasher.update(self.end_column.to_le_bytes());
        hasher.finalize().into()
    }
}

impl From<Span> for Meta {
    fn from(span: Span) -> Self {
        let start = span.start();
        let end = span.end();
        Self {
            file: span.file(),
            start_line: start.line() as u64,
            start_column: start.column() as u64,
            end_line: end.line() as u64,
            end_column: end.column() as u64,
        }
    }
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Shape {
    Struct(Sequence),
    Enum(Vec<(String, Sequence)>),
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Sequence {
    Object(Vec<(String, Entry)>),
    Tuple(Vec<Entry>),
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Lambda {
    pub meta: Meta,
    pub name: String,
    pub entry: Entry,
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Entry {
    pub factor: Factor,
    pub quantifier: Option<Quantifier>,
}

#[derive(SchemaRead, SchemaWrite)]
pub enum Factor {
    Literal(String),
    Set(Set),
    Term(Vec<Vec<Entry>>),
    Shard(String),
}

#[derive(SchemaRead, SchemaWrite)]
pub struct Quantifier {
    pub repeater: Repeater,
    pub lazy: bool,
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
