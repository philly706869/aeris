use std::collections::{BTreeMap, BTreeSet, VecDeque};

use rustc_hash::FxHashMap;

use super::mapping::Selected;
use super::{Cluster, ExtractError, MappingNode, ShiftRange, Symbol, Table};
use crate::shard::{ShardField, StaticShard};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// Farthest consumed UTF-8 byte offset.
    pub offset: usize,
}
impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "input is not accepted at byte {}", self.offset)
    }
}
impl std::error::Error for ParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Label {
    Symbol(Symbol),
    // `dot` is the start of the shared RHS suffix, constructed backwards.
    Intermediate { rule: usize, dot: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Key {
    label: Label,
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packed {
    rule: usize,
    split: usize,
    children: Vec<usize>,
}
struct Node {
    key: Key,
    alternatives: BTreeSet<Packed>,
}
#[derive(Default)]
struct Forest {
    nodes: Vec<Node>,
    ids: FxHashMap<Key, usize>,
}
impl Forest {
    fn node(&mut self, key: Key) -> usize {
        *self.ids.entry(key).or_insert_with(|| {
            let id = self.nodes.len();
            self.nodes.push(Node {
                key,
                alternatives: BTreeSet::new(),
            });
            id
        })
    }

    /// Productive alternatives only, then a topological cycle test reachable
    /// from the accepted root. Neither pass recursively walks the Rust stack.
    fn choices(&self, root: usize) -> Result<Vec<Vec<Packed>>, ExtractError> {
        let mut productive = vec![false; self.nodes.len()];
        loop {
            let mut changed = false;
            for (id, node) in self.nodes.iter().enumerate() {
                if !productive[id]
                    && (matches!(node.key.label, Label::Symbol(Symbol::Terminal(_)))
                        || node
                            .alternatives
                            .iter()
                            .any(|p| p.children.iter().all(|&c| productive[c])))
                {
                    productive[id] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        if !productive[root] {
            return Err(ExtractError::InvalidMapping);
        }
        let choices: Vec<Vec<_>> = self
            .nodes
            .iter()
            .map(|node| {
                node.alternatives
                    .iter()
                    .filter(|p| p.children.iter().all(|&c| productive[c]))
                    .cloned()
                    .collect()
            })
            .collect();
        let mut reachable = vec![false; self.nodes.len()];
        let mut pending = vec![root];
        let mut indegree = vec![0usize; self.nodes.len()];
        while let Some(id) = pending.pop() {
            if core::mem::replace(&mut reachable[id], true) {
                continue;
            }
            for packed in &choices[id] {
                for &child in &packed.children {
                    indegree[child] += 1;
                    pending.push(child);
                }
            }
        }
        let total = reachable.iter().filter(|&&v| v).count();
        let mut queue: VecDeque<_> = (0..self.nodes.len())
            .filter(|&id| reachable[id] && indegree[id] == 0)
            .collect();
        let mut visited = 0;
        while let Some(id) = queue.pop_front() {
            visited += 1;
            for packed in &choices[id] {
                for &child in &packed.children {
                    indegree[child] -= 1;
                    if indegree[child] == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
        if visited != total {
            return Err(ExtractError::InfiniteDerivations);
        }
        Ok(choices)
    }
}

struct Head {
    state: usize,
    offset: usize,
    // (predecessor, forest symbol). Different labels never collapse.
    edges: BTreeSet<(usize, usize)>,
}
#[derive(Default)]
struct Parser {
    forest: Forest,
    heads: Vec<Head>,
    current: BTreeMap<usize, usize>,
}
impl Parser {
    fn head(&mut self, state: usize, offset: usize) -> usize {
        *self.current.entry(state).or_insert_with(|| {
            let id = self.heads.len();
            self.heads.push(Head {
                state,
                offset,
                edges: BTreeSet::new(),
            });
            id
        })
    }

    fn reduce(&mut self, table: &Table, offset: usize, lookahead: Option<char>) {
        loop {
            let mut changed = false;
            let heads: Vec<_> = self.current.values().copied().collect();
            for head in heads {
                for &rule_id in &table.states[self.heads[head].state].reductions {
                    let rule = &table.rules[rule_id];
                    // One shared suffix per predecessor at each pop depth.
                    // Alternative paths become packed children, never a product
                    // of fully expanded stack paths.
                    let mut frontier = BTreeMap::from([(head, None)]);
                    for dot in (0..rule.rhs.len()).rev() {
                        let mut next = BTreeMap::new();
                        for (top, suffix) in frontier {
                            for &(ancestor, symbol) in &self.heads[top].edges {
                                if self.forest.nodes[symbol].key.label
                                    != Label::Symbol(rule.rhs[dot])
                                {
                                    continue;
                                }
                                let start = self.heads[ancestor].offset;
                                let intermediate = self.forest.node(Key {
                                    label: Label::Intermediate { rule: rule_id, dot },
                                    start,
                                    end: offset,
                                });
                                let mut children = vec![symbol];
                                if let Some(suffix) = suffix {
                                    children.push(suffix);
                                }
                                let split = self.forest.nodes[symbol].key.end;
                                self.forest.nodes[intermediate].alternatives.insert(Packed {
                                    rule: rule_id,
                                    split,
                                    children,
                                });
                                next.insert(ancestor, Some(intermediate));
                            }
                        }
                        frontier = next;
                    }
                    for (ancestor, suffix) in frontier {
                        let state = &table.states[self.heads[ancestor].state];
                        if let Some(&target) = state.gotos.get(&rule.lhs) {
                            if !table.states[target].first.permits(lookahead) {
                                continue;
                            }
                            let start = self.heads[ancestor].offset;
                            let symbol = self.forest.node(Key {
                                label: Label::Symbol(Symbol::Nonterminal(rule.lhs)),
                                start,
                                end: offset,
                            });
                            self.forest.nodes[symbol].alternatives.insert(Packed {
                                rule: rule_id,
                                split: offset,
                                children: suffix.into_iter().collect(),
                            });
                            let target = self.head(target, offset);
                            changed |= self.heads[target].edges.insert((ancestor, symbol));
                        }
                    }
                }
            }
            // New packed alternatives under existing labels are immediately
            // visible through sharing; only new GSS edges require another pass.
            if !changed {
                break;
            }
        }
    }

    fn run(mut self, table: &Table, input: &str) -> Result<(Forest, usize), ParseError> {
        self.head(0, 0);
        for (offset, ch) in input.char_indices() {
            self.reduce(table, offset, Some(ch));
            let heads = core::mem::take(&mut self.current);
            for (state, head) in heads {
                for &target in ShiftRange::lookup(&table.states[state].dispatch, ch) {
                    let terminal = table.states[state].shift_symbols[&target];
                    let end = offset + ch.len_utf8();
                    let symbol = self.forest.node(Key {
                        label: Label::Symbol(Symbol::Terminal(terminal)),
                        start: offset,
                        end,
                    });
                    let next = self.head(target, end);
                    self.heads[next].edges.insert((head, symbol));
                }
            }
            if self.current.is_empty() {
                return Err(ParseError { offset });
            }
        }
        self.reduce(table, input.len(), None);
        for (&state, &head) in &self.current {
            if table.states[state].accept {
                for &(ancestor, symbol) in &self.heads[head].edges {
                    if ancestor == 0
                        && self.forest.nodes[symbol].key.label
                            == Label::Symbol(table.rules[0].rhs[0])
                    {
                        return Ok((self.forest, symbol));
                    }
                }
            }
        }
        Err(ParseError {
            offset: input.len(),
        })
    }
}

pub struct Parsed<'c, 'i, S: StaticShard> {
    cluster: &'c Cluster<S>,
    input: &'i str,
    forest: Forest,
    root: usize,
}
impl<'c, 'i, S: StaticShard> Parsed<'c, 'i, S> {
    pub(super) fn new(cluster: &'c Cluster<S>, input: &'i str) -> Result<Self, ParseError> {
        let (forest, root) = Parser::default().run(&cluster.table, input)?;
        Ok(Self {
            cluster,
            input,
            forest,
            root,
        })
    }

    /// Enumerates derivations without applying greedy/lazy ranking or value
    /// deduplication. A productive accepted cycle is an extraction error.
    pub fn results(&self) -> Result<Results<'_, 'c, 'i, S>, ExtractError> {
        Ok(Results {
            parsed: self,
            choices: self.forest.choices(self.root)?,
            path: Vec::new(),
            done: false,
        })
    }
}

pub struct Results<'p, 'c, 'i, S: StaticShard> {
    parsed: &'p Parsed<'c, 'i, S>,
    choices: Vec<Vec<Packed>>,
    path: Vec<usize>,
    done: bool,
}
impl<'i, S: StaticShard> Iterator for Results<'_, '_, 'i, S> {
    type Item = Result<ShardField<'i, S>, ExtractError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        // Preorder choice digits form a variable-length odometer. Only one
        // selected derivation and its output are constructed per next() call.
        let mut nodes: Vec<Selected> = Vec::new();
        let mut pending: Vec<(usize, Option<usize>)> = vec![(self.parsed.root, None)];
        let mut arities = Vec::new();
        while let Some((forest_id, parent)) = pending.pop() {
            let node = &self.parsed.forest.nodes[forest_id];
            let id = nodes.len();
            let alternatives = &self.choices[forest_id];
            let packed = if alternatives.is_empty() {
                None
            } else {
                let digit = arities.len();
                if self.path.len() <= digit {
                    self.path.push(0);
                }
                arities.push(alternatives.len());
                Some(&alternatives[self.path[digit]])
            };
            nodes.push(Selected {
                symbol: match node.key.label {
                    Label::Symbol(symbol) => Some(symbol),
                    _ => None,
                },
                start: node.key.start,
                end: node.key.end,
                rule: packed.map(|p| p.rule),
                children: Vec::new(),
            });
            if let Some(parent) = parent {
                nodes[parent].children.push(id);
            }
            if let Some(packed) = packed {
                pending.extend(packed.children.iter().rev().map(|&child| (child, Some(id))));
            }
        }
        self.done = true;
        for digit in (0..arities.len()).rev() {
            if self.path[digit] + 1 < arities[digit] {
                self.path[digit] += 1;
                self.path.truncate(digit + 1);
                self.done = false;
                break;
            }
        }
        let mapping = MappingNode {
            input: self.parsed.input,
            nodes: &nodes,
            table: &self.parsed.cluster.table,
            id: 0,
        };
        Some(mapping.reference::<S>())
    }
}
impl<S: StaticShard> core::iter::FusedIterator for Results<'_, '_, '_, S> {}
