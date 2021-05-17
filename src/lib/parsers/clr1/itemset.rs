
use super::item::Item;
use crate::lib::parsers::common::Symbol;

use std::collections::HashSet;

pub struct ItemSet<'r> {
    kernel: HashSet<Item<'r>>,
    closure: HashSet<Item<'r>>
}

impl<'r> ItemSet<'r> {

    pub fn new() -> Self {
        return Self {
            kernel: HashSet::new(),
            closure: HashSet::new()
        };
    }

    pub fn from_kernel(kernel_items: Vec<Item<'r>>) -> Self {
        let mut kernel = HashSet::new();

        for item in kernel_items {
            kernel.insert(item);
        }

        return Self {
            kernel,
            closure: HashSet::new()
        }
    }

    pub fn get_active_symbols(&self) -> HashSet<Symbol> {
        let mut symbols = HashSet::new();
        for item in self.all() {
            if let Some(sym) = item.get_active_symbol() {
                symbols.insert(sym.clone());
            }
        }

        return symbols;
    }
   
    pub fn get_kernel(&self) -> &HashSet<Item<'r>> {
        return &self.kernel;
    }
    
    pub fn extend_closure(&mut self, closure: HashSet<Item<'r>>) {
        self.closure.extend(closure);
    }

    pub fn all(&self) -> std::collections::hash_set::Union<Item<'r>, std::collections::hash_map::RandomState> {
        return self.kernel.union(&self.closure);
    }

    pub fn insert_kernel(&mut self, item: Item<'r>) -> bool {
        return self.kernel.insert(item);
    }

    pub fn is_same_kernel(&self, other_set: &ItemSet<'r>) -> bool {
        return self.kernel == other_set.kernel;
    } 

}
