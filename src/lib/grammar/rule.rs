
use super::Symbol;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Rule<Terminal, NonTerminal>{
    lhs: NonTerminal,
    symbols: Vec<Symbol<Terminal, NonTerminal>>
}

impl<Terminal: Clone, NonTerminal: Clone> Rule<Terminal, NonTerminal>{
    pub fn new(lhs: NonTerminal, symbols: Vec<Symbol<Terminal, NonTerminal>>) -> Self {
        return Self{ lhs, symbols }
    }

    pub fn get_lhs(self: &Self) -> &NonTerminal {
        return &self.lhs;
    }

    pub fn get_lhs_as_sym(&self) -> Symbol<Terminal, NonTerminal> {
        return Symbol::NonTerminal(self.lhs.clone());
    }

    pub fn get_symbols(self: &Self) -> &Vec<Symbol<Terminal, NonTerminal>> {
        return &self.symbols;
    }
}