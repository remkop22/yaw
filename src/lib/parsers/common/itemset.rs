
use super::{ Symbol, Item };

use std::collections::HashSet;
use std::hash::Hash;

pub struct ItemSet<I> {
    kernel: HashSet<I>,
    closure: HashSet<I>
}

impl<I: Eq + Hash + Item> ItemSet<I> {

    pub fn new() -> Self {
        return Self {
            kernel: HashSet::new(),
            closure: HashSet::new()
        };
    }

    pub fn from_kernel(kernel_items: Vec<I>) -> Self {
        let mut kernel = HashSet::new();

        for item in kernel_items {
            kernel.insert(item);
        }

        return Self {
            kernel,
            closure: HashSet::new()
        }
    }

    pub fn active_symbols(&self) -> HashSet<Symbol> {
        let mut symbols = HashSet::new();
        for item in self.all() {
            if let Some(sym) = item.active_symbol() {
                symbols.insert(sym.clone());
            }
        }

        return symbols;
    }
   
    pub fn kernel(&self) -> &HashSet<I> {
        return &self.kernel;
    }
    
    pub fn extend_closure(&mut self, closure: HashSet<I>) {
        self.closure.extend(closure);
    }

    pub fn all(&self) -> std::collections::hash_set::Union<I, std::collections::hash_map::RandomState> {
        return self.kernel.union(&self.closure);
    }

    pub fn insert_kernel(&mut self, item: I) -> bool {
        return self.kernel.insert(item);
    }

    pub fn is_same_kernel(&self, other_set: &ItemSet<I>) -> bool {
        return self.kernel == other_set.kernel;
    } 

}
