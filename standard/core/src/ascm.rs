//! AERIS Standard Code Model

use std::collections::HashMap;

trait PropAccessable {
    fn prop(&self, name: &str) -> Option<&dyn Prop>;

    fn props_hint(&self) -> Vec<&str>;
}

trait Prop {}

pub struct Module {
    prop_table: HashMap<String, Box<dyn Prop>>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            prop_table: HashMap::new(),
        }
    }
}

impl Prop for Module {}

impl PropAccessable for Module {
    fn prop(&self, name: &str) -> Option<&dyn Prop> {
        self.prop_table.get(name).map(Box::as_ref)
    }

    fn props_hint(&self) -> Vec<&str> {
        self.prop_table.keys().map(String::as_str).collect()
    }
}

pub struct Function {
    return_type: (),
    params: Vec<()>,
    expression: Box<dyn Expression>,
}

impl Prop for Function {}

pub trait Expression {}

pub struct Scope {
    expressions: Vec<Box<dyn Expression>>,
}

impl Expression for Scope {}

pub struct Invoke {
    invokable: (),
}

pub struct Class {
    parents: HashMap<String, ()>,
    fields: HashMap<String, ()>,
}

impl Prop for Class {}

impl PropAccessable for Class {
    fn prop(&self, name: &str) -> Option<&dyn Prop> {
        todo!()
    }

    fn props_hint(&self) -> Vec<&str> {
        todo!()
    }
}
