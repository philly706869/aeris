use core::any::TypeId;

use super::{MappingTask, Symbol, Table};
use crate::shard::{Shard, ShardCore};

#[derive(Clone, Copy, Debug)]
pub(super) enum Shape {
    Transparent,
    Span,
    Sequence,
    Alternative,
    Option,
    Repeat,
    Reference(TypeId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractError {
    InfiniteDerivations,
    InvalidMapping,
}

impl core::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::InfiniteDerivations => "accepted input has infinitely many derivations",
            Self::InvalidMapping => "derivation does not match the shard mapping",
        })
    }
}
impl std::error::Error for ExtractError {}

pub(super) struct Selected {
    pub symbol: Option<Symbol>,
    pub start: usize,
    pub end: usize,
    pub rule: Option<usize>,
    pub children: Vec<usize>,
}

/// A checked view of one selected derivation. Outputs may borrow only `input`.
#[derive(Clone, Copy)]
pub struct MappingNode<'a, 'i> {
    pub(super) input: &'i str,
    pub(super) nodes: &'a [Selected],
    pub(super) table: &'a Table,
    pub(super) id: usize,
}

impl<'a, 'i> MappingNode<'a, 'i> {
    fn at(self, id: usize) -> Self {
        Self { id, ..self }
    }

    fn raw_children(self) -> Vec<Self> {
        let mut pending: Vec<_> = self.nodes[self.id].children.iter().rev().copied().collect();
        let mut children = Vec::new();
        while let Some(id) = pending.pop() {
            if self.nodes[id].symbol.is_none() {
                pending.extend(self.nodes[id].children.iter().rev().copied());
            } else {
                children.push(self.at(id));
            }
        }
        children
    }

    fn shape(self) -> Result<Shape, ExtractError> {
        match self.nodes[self.id].symbol {
            Some(Symbol::Nonterminal(id)) => self
                .table
                .shapes
                .get(id)
                .copied()
                .ok_or(ExtractError::InvalidMapping),
            _ => Err(ExtractError::InvalidMapping),
        }
    }

    fn resolved(mut self) -> Result<Self, ExtractError> {
        while matches!(self.shape()?, Shape::Transparent) {
            let children = self.raw_children();
            if children.len() != 1 {
                return Err(ExtractError::InvalidMapping);
            }
            self = children[0];
        }
        Ok(self)
    }

    pub fn slice(self) -> Result<&'i str, ExtractError> {
        let node = &self.nodes[self.id];
        self.input
            .get(node.start..node.end)
            .ok_or(ExtractError::InvalidMapping)
    }

    pub fn field(self, index: usize, count: usize) -> Result<Self, ExtractError> {
        // The macro removes singleton sequences from the recognition grammar.
        if count == 1 && index == 0 {
            return Ok(self);
        }
        let node = self.resolved()?;
        if !matches!(node.shape()?, Shape::Sequence) {
            return Err(ExtractError::InvalidMapping);
        }
        let children = node.raw_children();
        if children.len() != count {
            return Err(ExtractError::InvalidMapping);
        }
        children
            .get(index)
            .copied()
            .ok_or(ExtractError::InvalidMapping)
    }

    pub fn sequence(self, count: usize) -> Result<(), ExtractError> {
        if count == 1 {
            return Ok(());
        }
        let node = self.resolved()?;
        if !matches!(node.shape()?, Shape::Sequence) || node.raw_children().len() != count {
            return Err(ExtractError::InvalidMapping);
        }
        Ok(())
    }

    pub fn alternative(self) -> Result<(usize, Self), ExtractError> {
        let node = self.resolved()?;
        if !matches!(node.shape()?, Shape::Alternative) {
            return Err(ExtractError::InvalidMapping);
        }
        let rule = node.nodes[node.id]
            .rule
            .ok_or(ExtractError::InvalidMapping)?;
        let children = node.raw_children();
        if children.len() != 1 {
            return Err(ExtractError::InvalidMapping);
        }
        Ok((node.table.rules[rule].branch, children[0]))
    }

    pub fn optional(self) -> Result<Option<Self>, ExtractError> {
        let node = self.resolved()?;
        if !matches!(node.shape()?, Shape::Option) {
            return Err(ExtractError::InvalidMapping);
        }
        let children = node.raw_children();
        match children.as_slice() {
            [] => Ok(None),
            [child] => Ok(Some(*child)),
            _ => Err(ExtractError::InvalidMapping),
        }
    }

    pub fn repeated(self) -> Result<Vec<Self>, ExtractError> {
        let mut node = self.resolved()?;
        let mut items = Vec::new();
        loop {
            if !matches!(node.shape()?, Shape::Repeat) {
                return Err(ExtractError::InvalidMapping);
            }
            let children = node.raw_children();
            match children.as_slice() {
                [] => return Ok(items),
                [item, tail] => {
                    items.push(*item);
                    node = tail.resolved()?;
                }
                _ => return Err(ExtractError::InvalidMapping),
            }
        }
    }

    pub fn reference<T: Shard>(self) -> Result<<T::Core as ShardCore>::Output<'i>, ExtractError> {
        self.reference_task::<T>().run()
    }

    /// Defer the cross-shard call so recursive grammars do not recurse through
    /// Rust function calls. Only the driver executes a task's continuation.
    pub fn reference_task<'m, T: Shard>(self) -> MappingTask<'m, <T::Core as ShardCore>::Output<'i>>
    where
        'a: 'm,
        'i: 'm,
    {
        MappingTask::defer(move || {
            let node = self.resolved()?;
            if !matches!(node.shape()?, Shape::Reference(id) if id == TypeId::of::<T::Core>()) {
                return Err(ExtractError::InvalidMapping);
            }
            let children = node.raw_children();
            if children.len() != 1 {
                return Err(ExtractError::InvalidMapping);
            }
            Ok(T::Core::map(children[0]))
        })
    }
}
