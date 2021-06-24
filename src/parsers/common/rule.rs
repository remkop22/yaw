
use super::symbol::{ Symbol };

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Rule<Term, NonTerm> {
    lhs: NonTerm,
    symbols: Vec<Symbol<Term, NonTerm>>,
    keep_all: bool,
    priority: usize
}

impl<Term, NonTerm> Rule<Term, NonTerm> {

    pub fn new(lhs: NonTerm, symbols: Vec<Symbol<Term, NonTerm>>, keep_all: bool, priority: usize) -> Self {
        return Self{ lhs, symbols, keep_all, priority }
    }

    pub fn lhs(&self) -> &NonTerm {
        return &self.lhs;
    }

    pub fn lhs_as_sym(&self) -> Symbol<Term, NonTerm> {
        return Symbol::NonTerminal(self.lhs);
    }

    pub fn symbols(&self) -> &Vec<Symbol<Term, NonTerm>> {
        return &self.symbols;
    }

}

impl<'a, Term, NonTerm> IntoIterator for &'a Rule<Term, NonTerm> {
    type Item = &'a Symbol<Term, NonTerm>;
    type IntoIter = std::slice::Iter<'a, Symbol<Term, NonTerm>>;

    fn into_iter(self) -> Self::IntoIter {
        return self.symbols.iter();
    }
}