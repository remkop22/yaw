use std::cmp::{ Eq, PartialEq };
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Symbol<Terminal, NonTerminal> {
    Terminal(Terminal, Option<&'static str>),
    NonTerminal(NonTerminal),
    EndOfTokenStream,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Rule<Terminal, NonTerminal>{
    lhs: NonTerminal,
    symbols: Vec<Symbol<Terminal, NonTerminal>>
}

impl<Terminal, NonTerminal> Rule<Terminal, NonTerminal>{
    pub fn new(lhs: NonTerminal, symbols: Vec<Symbol<Terminal, NonTerminal>>) -> Self {
        return Self{ lhs, symbols }
    }

    pub fn get_lhs(self: &Self) -> &NonTerminal {
        return &self.lhs;
    }

    pub fn get_symbols(self: &Self) -> &Vec<Symbol<Terminal, NonTerminal>> {
        return &self.symbols;
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Item<'rule, Terminal, NonTerminal>{
    rule: &'rule Rule<Terminal, NonTerminal>,
    index: usize,
    look_ahead: Symbol<Terminal, NonTerminal>
}

impl<'rule, Terminal: Clone, NonTerminal: Clone> Item<'rule, Terminal, NonTerminal> {

    pub fn new(
        rule: &'rule Rule<Terminal, NonTerminal>, 
        index: usize, 
        look_ahead: Symbol<Terminal, NonTerminal>
    ) -> Self {
        return Self {
            rule,
            index,
            look_ahead
        }
    }

    pub fn get_lhs(&self) -> &NonTerminal {
        return self.rule.get_lhs();
    }

    pub fn get_rule(&self) -> &Rule<Terminal, NonTerminal> {
        return self.rule;
    }

    pub fn is_active(self: &Self) -> bool {
        return self.index < self.rule.get_symbols().len();
    }

    pub fn get_look_ahead(&self) -> &Symbol<Terminal, NonTerminal> {
        return &self.look_ahead;
    }

    pub fn get_active_symbol(self: &Self) -> Option<&Symbol<Terminal, NonTerminal>> {
        return self.rule.get_symbols().get(self.index);
    }

    pub fn goto(self: &Self) -> Option<Item<'rule, Terminal, NonTerminal>> {
        if self.is_active() {
            return Some(Item::new(self.rule, self.index + 1, self.look_ahead.clone()));
        } else {
            return None;
        }
    }

}


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

    pub fn get_active_symbols(&self) -> HashSet<Symbol<Terminal, NonTerminal>> {
        let mut symbols = HashSet::new();
        for item in self.all() {
            if let Some(sym) = item.get_active_symbol() {
                symbols.insert(sym.clone());
            }
        }

        return symbols;
    }
    
    pub fn get_active_symbols_as_ref(&self) -> HashSet<&Symbol<Terminal, NonTerminal>> {
        let mut symbols = HashSet::new();
        for item in self.all() {
            if let Some(sym) = item.get_active_symbol() {
                symbols.insert(sym);
            }
        }

        return symbols;
    }

    pub fn get_kernel(&self) -> &HashSet<Item<'rules, Terminal, NonTerminal>> {
        return &self.kernel;
    }
    
    pub fn get_closure(&self) -> &HashSet<Item<'rules, Terminal, NonTerminal>> {
        return &self.closure;
    }

    pub fn all(&self) -> std::collections::hash_set::Union<Item<'rules, Terminal, NonTerminal>, std::collections::hash_map::RandomState> {
        return self.kernel.union(&self.closure);
    }

    pub fn insert_kernel(&mut self, item: Item<'rules, Terminal, NonTerminal>) -> bool {
        return self.kernel.insert(item);
    }

    pub fn insert_closure(&mut self, item: Item<'rules, Terminal, NonTerminal>) -> bool {
        return self.kernel.insert(item);
    }

    pub fn is_same_kernel(&self, other_set: &ItemSet<'rules, Terminal, NonTerminal>) -> bool {
        return self.kernel == other_set.kernel;
    } 

}
