
use super::symbol::{ Symbol, NonTerminal };

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Rule {
    lhs: NonTerminal,
    symbols: Vec<Symbol>,
    keep_all: bool,
    priority: usize
}

impl Rule {

    pub fn new(lhs: NonTerminal, symbols: Vec<Symbol>, keep_all: bool, priority: usize) -> Self {
        return Self{ lhs, symbols, keep_all, priority }
    }

    pub fn get_lhs(self: &Self) -> &NonTerminal {
        return &self.lhs;
    }

    pub fn get_lhs_as_sym(&self) -> Symbol {
        return Symbol::NonTerminal(self.lhs.clone());
    }

    pub fn get_symbols(self: &Self) -> &Vec<Symbol> {
        return &self.symbols;
    }

}

impl<'a> IntoIterator for &'a Rule {
    type Item = &'a Symbol;
    type IntoIter = std::slice::Iter<'a, Symbol>;

    fn into_iter(self) -> Self::IntoIter {
        return self.symbols.iter();
    }
}