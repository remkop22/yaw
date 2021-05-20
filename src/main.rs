
mod lib;
use lib::common::{Symbol, Rule, Terminal, NonTerminal, LRAnalyser};
use lib::parsers::clr1::CLR1Analyser;


fn main() {

    let statement = &NonTerminal::new("statement".to_string());
    let operation = &NonTerminal::new("operation".to_string());
    let operator = &Terminal::new("operator".to_string(), true);
    let number = &Terminal::new("number".to_string(), true);
    let semicolon = &Terminal::new("semicolon".to_string(), true);
    
    let rules = vec![ 
        Rule::new(statement.clone(), vec![
            Symbol::NonTerminal(statement.clone()),
            Symbol::Terminal(semicolon.clone())
        ], true, 0),
        
        Rule::new(operation.clone(), vec![
            Symbol::Terminal(number.clone()),
            Symbol::Terminal(operator.clone()),
            Symbol::Terminal(number.clone())
        ], true, 0),
    ];

    let analyser = CLR1Analyser::new(&rules, &rules[0]);
    let table = analyser.table();
    
    println!("{:?}", table.actions().keys());
    println!("{:?}", table.gotos().keys());

}

