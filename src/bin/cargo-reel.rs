use reel::analysis::Analyser;
use reel::common::{Grammar, Rule, Symbol};
use reel::generator::generate;
use std::env;


#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum TerminalSymbols {
	Identifier,
	Separator(&'static str),
	EOF
}

impl reel::common::Terminal for TerminalSymbols {
	fn eof() -> Self {
		Self::EOF
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum NonTerminalSymbols {
	Expression,
	FunctionCall,
	FunctionArgs,
}

use NonTerminalSymbols::*;
use Symbol::*;
use TerminalSymbols::*;

fn main() {
	

	let grammar = Grammar::new(vec![
		Rule::new(Expression, vec![Terminal(Identifier)]),
		Rule::new(Expression, vec![NonTerminal(FunctionCall)]),
		Rule::new(
			FunctionCall,
			vec![
				NonTerminal(Expression),
				Terminal(Separator("(")),
				NonTerminal(FunctionArgs),
				Terminal(Separator(")")),
			],
		),
		Rule::new(
			FunctionCall,
			vec![
				NonTerminal(Expression),
				Terminal(Separator("(")),
				Terminal(Separator(")")),
			],
		),
		Rule::new(
			FunctionArgs,
			vec![
				NonTerminal(FunctionArgs),
				Terminal(Separator(",")),
				NonTerminal(Expression),
			],
		),
		Rule::new(
			FunctionArgs,
			vec![
				NonTerminal(Expression),
			],
		),
	], 0);
	
	let args: Vec<String> = env::args().collect();
 	let file_name = args.get(2).unwrap();

	generate(file_name, Analyser::new(&grammar).table()).unwrap();

}
