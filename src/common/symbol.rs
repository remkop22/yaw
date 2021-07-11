use std::fmt::Debug;
use std::hash::Hash;

pub trait Terminal: Eq + Hash + Copy + Debug {
    fn eof() -> Self;
}

pub trait NonTerminal: Eq + Hash + Copy + Debug {}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Symbol<T, NT> {
    Terminal(T),
    NonTerminal(NT),
}

impl<T, NT> Symbol<T, NT>
where
    T: Copy,
    NT: Copy,
{
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Terminal(_) => true,
            Self::NonTerminal(_) => false,
        }
    }

    pub fn terminal(&self) -> Option<T> {
        match self {
            Self::Terminal(term) => Some(*term),
            Self::NonTerminal(_) => None,
        }
    }

    pub fn non_terminal(&self) -> Option<NT> {
        match self {
            Self::NonTerminal(nonterm) => Some(*nonterm),
            Self::Terminal(_) => None,
        }
    }
}
