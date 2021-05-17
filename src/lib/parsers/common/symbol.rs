

pub const EOF: &str = "$EOF";

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Terminal {
    name: String,
    keep: bool
}

impl Terminal {
    pub fn new(name: String, keep: bool) -> Self {
        return Self{name, keep};
    }
}


#[derive(Eq, PartialEq, Hash, Clone)]
pub struct NonTerminal {
    name: String
}

impl NonTerminal {
    pub fn new(name: String) -> Self {
        return Self{name};
    }
}



#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Symbol{
    Terminal(Terminal),
    NonTerminal(NonTerminal)
}

impl Symbol {

    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Terminal(_) => true,
            Self::NonTerminal(_) => false
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Terminal(t) => &t.name[..],
            Self::NonTerminal(nt) => &nt.name[..]
        }
    }

}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal(t) => f.write_str(&format!("<{}>", t.name)[..]),
            Self::NonTerminal(nt) => f.write_str(&nt.name[..])
        }
    }
}