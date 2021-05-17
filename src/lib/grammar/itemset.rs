
use super::item::Item;
use super::Symbol;

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct ItemSet<'rules, Terminal, NonTerminal> {
    kernel: HashSet<Item<'rules, Terminal, NonTerminal>>,
    closure: HashSet<Item<'rules, Terminal, NonTerminal>>
}

impl<'rules, Terminal: Hash + Eq + Clone, NonTerminal: Hash + Eq + Clone> ItemSet<'rules, Terminal, NonTerminal> {

    pub fn new() -> Self {
        return Self {
            kernel: HashSet::new(),
            closure: HashSet::new()
        };
    }

    pub fn from_kernel(kernel_items: Vec<Item<'rules, Terminal, NonTerminal>>) -> Self {
        let mut kernel = HashSet::new();

        for item in kernel_items {
            kernel.insert(item);
        }

        return Self {
            kernel,
            closure: HashSet::new()
        }
    }

    pub fn get_active_symbols(&self) -> HashSet<Symbol<Terminal, NonTerminal>> {
        let mut symbols = HashSet::new();
        for item in self.all() {
            if let Some(sym) = item.get_active_symbol() {
                symbols.insert(sym.clone());
            }
        }

        return symbols;
    }
   
    pub fn get_kernel(&self) -> &HashSet<Item<'rules, Terminal, NonTerminal>> {
        return &self.kernel;
    }
    
    pub fn extend_closure(&mut self, closure: HashSet<Item<'rules, Terminal, NonTerminal>>) {
        self.closure.extend(closure);
    }

    pub fn all(&self) -> std::collections::hash_set::Union<Item<'rules, Terminal, NonTerminal>, std::collections::hash_map::RandomState> {
        return self.kernel.union(&self.closure);
    }

    pub fn insert_kernel(&mut self, item: Item<'rules, Terminal, NonTerminal>) -> bool {
        return self.kernel.insert(item);
    }

    pub fn is_same_kernel(&self, other_set: &ItemSet<'rules, Terminal, NonTerminal>) -> bool {
        return self.kernel == other_set.kernel;
    } 

}
