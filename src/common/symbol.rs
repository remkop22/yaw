
use std::hash::Hash;
use std::fmt::Debug;

pub trait Terminal: Eq + Hash + Copy + Debug {
	fn eof() -> Self;
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Symbol<T, NT>{
    Terminal(T),
    NonTerminal(NT),
}

impl<T, NT> Symbol<T, NT> {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Terminal(_) => true,
            Self::NonTerminal(_) => false
        }
    }

}
