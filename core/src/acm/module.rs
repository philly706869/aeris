use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::acm::{Function, Global, uid::Uid};

#[derive(Debug, Clone)]
pub struct Module<'f, 'g> {
    pub functions: HashMap<&'f Function, String>,
    pub globals: HashSet<&'g Global>,
}

impl<'f, 'g> Module<'f, 'g> {}
