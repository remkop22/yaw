
mod lib;
use lib::common::{Symbol, Rule, Terminal, NonTerminal};
use lib::parsers::clr1::CLR1Analyser;


fn main() {

    let rules = vec![ 
        Rule::new(NonTerminal::new("".to_string()), vec![
            Symbol::Terminal(Terminal::new("".to_string(), true) )
        ], true, 0),
    ];

    let analyser = CLR1Analyser::new(&rules, NonTerminal::new("".to_string()));
    let table = analyser.get_table();
    println!("{:?}", table.get_actions().keys());
    println!("{:?}", table.get_gotos().keys());

}

