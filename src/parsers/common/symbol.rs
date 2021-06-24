
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Symbol<Term, NonTerm>{
    Terminal(Term),
    NonTerminal(NonTerm),
	EOF
}

impl<Term, NonTerm> Symbol<Term, NonTerm> {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Terminal(_) => true,
			Self::EOF => true,
            Self::NonTerminal(_) => false
        }
    }

}
