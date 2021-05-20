
mod lib;
use lib::common::{Symbol, Rule, Terminal, NonTerminal, LRAnalyser};
use lib::parsers::clr1::CLR1Analyser;


fn main() {

    let rules = vec![ 
        Rule::new(NonTerminal::new("".to_string()), vec![
            Symbol::Terminal(Terminal::new("".to_string(), true) )
        ], true, 0),
    ];

    let analyser = CLR1Analyser::new(&rules, &rules[0]);
    let table = analyser.table();

    println!("{:?}", table.actions().keys());
    println!("{:?}", table.gotos().keys());

}

