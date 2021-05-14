
use regex::Regex;

mod lib;
use lib::*;

fn main() {

    let token_types = &vec![
        TokenType::new(String::from("identifier"), Regex::new("^hmm$").unwrap()),
        TokenType::new(String::from("seperator"), vec![".", ",", "{", "}", "(", ")"])
    ];

    let input = &mut "..hmm...".as_bytes();
    let tokenizer = Tokenizer::new(input, token_types);
    for result in tokenizer {
        if let Ok(token) = result {
            println!("'{}', {}", token.get_value(), token.get_kind().get_name())
        } else if let Err(msg) = result {
            panic!("{}", msg);
        }
    }
}
