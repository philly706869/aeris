use proc_macro2::Ident;
use quote::format_ident;
use rustc_hash::FxHashSet;

#[derive(Default)]
pub struct Names {
    used: FxHashSet<String>,
}

impl Names {
    pub fn reserve(&mut self, ident: Ident) {
        let name = ident.to_string();
        let name = name.strip_prefix("r#").unwrap_or(&name).to_owned();
        self.used.insert(name);
    }

    pub fn generator<'a>(&'a self, namespace: &'a str) -> NameGenerator<'a> {
        NameGenerator::new(&self.used, namespace)
    }
}

pub struct NameGenerator<'a> {
    used: &'a FxHashSet<String>,
    namespace: &'a str,
    index: usize,
}

impl<'a> NameGenerator<'a> {
    fn new(used: &'a FxHashSet<String>, namespace: &'a str) -> Self {
        Self {
            used,
            namespace,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Ident {
        loop {
            let index = self.index;
            self.index = self
                .index
                .checked_add(1)
                .expect("generated identifier space exhausted");
            let name = format!("__xst_{}_{}", self.namespace, index);
            if !self.used.contains(&name) {
                return format_ident!("{name}");
            }
        }
    }
}
