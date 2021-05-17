
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExprType {
    Assignment
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

use TokenType::*;
use ExprType::*;
use Symbol::*;
use lib::generator::{ Symbol, Rule, generate_table };

macro_rules! use_reel {
    () => {
        include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    };
}

use_reel!();

//use generated::hello;

fn main() {

   let rules = vec![ Rule::new(Assignment, vec![Terminal(Keyword, None)]) ];
   let table = generate_table(&rules, Assignment);
   generated::hello();

}

pub mod lib;

#[cfg(test)]
mod tests {

    use super::lib::lexer::{ TokenPattern, Tokenizer };
    use super::TokenType;

    use regex::Regex;

    #[test]
    fn lexer(){

        let token_types = &[
            TokenPattern::new(TokenType::Keyword,       vec!["class", "if", "for", "const", "has", "is", "true", "false"]),
            TokenPattern::new(TokenType::Operator,      vec!["+", "=", "-", "/", "*", "&&", "||", "|", "&", "^"]),
            TokenPattern::new(TokenType::Seperator,     vec![".", ",", "{", "}", "(", ")"]),
            TokenPattern::new(TokenType::Number,        Regex::new(r"^[0-9]+$").unwrap()),
            TokenPattern::new(TokenType::String,        Regex::new(r#"^"[^"]*"$"#).unwrap()),
            TokenPattern::new(TokenType::Whitespace,    vec![" ", "\t", "\n"]),
            TokenPattern::new(TokenType::Identifier,    Regex::new(r"^[A-z_]+[0-z_]*$").unwrap()),
        ];

        let mut input = r#"if(true){print("Hello" + "World")}"#.as_bytes();

        let mut tokenizer = Tokenizer::new(&mut input, token_types);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Keyword);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Keyword);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Identifier);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::String);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Whitespace);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Operator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Whitespace);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::String);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().unwrap().unwrap().get_kind(), &TokenType::Seperator);
        assert_eq!(tokenizer.next().is_none(), true);

    }
}