
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExprType {
    Expression,
    Assignment,
    Statement
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    Operator,
    Seperator,
    Number,
    String,
    Identifier,
    Whitespace,
}

mod lib;
use TokenType::*;
use ExprType::*;
use lib::grammar::{Symbol::*, Rule};
use lib::table::{generate_table, Conflict};


fn main() {

    let rules = vec![ 
        Rule::new(Assignment, vec![Terminal(Identifier, None), Terminal(Operator, "=".into()), Terminal(Identifier, None)]),
        Rule::new(Statement, vec![NonTerminal(Assignment),Terminal(Seperator, ";".into())]),
    ];

    let table = generate_table(&rules, Statement, Conflict::Panic);
    println!("{:?}", table.get_actions().keys());
    println!("{:?}", table.get_gotos().keys());

}

