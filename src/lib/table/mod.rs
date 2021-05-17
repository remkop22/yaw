
mod table;
mod generator;

pub use table::{Table, Conflict};
pub use generator::generate_table;
use super::grammar::Rule;

pub enum Action<Terminal, NonTerminal> {
    Shift(usize),
    Reduce(Rule<Terminal, NonTerminal>),
    Accept,
    Error
}

