
pub mod item;
pub mod itemset;
pub mod rule;
pub mod token;

pub use rule::Rule;
pub use token::{Pattern, Token, TokenPattern};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Symbol<Terminal, NonTerminal> {
    Terminal(Terminal, Option<&'static str>),
    NonTerminal(NonTerminal),
    EndOfTokenStream,
}