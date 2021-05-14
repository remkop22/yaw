
use crate::lib::{ Token, TokenType };
use std::error;
use std::fmt;

pub struct Tokenizer<'reader, 'token_types> {
    token_types: &'token_types [TokenType],
    reader: &'reader mut dyn std::io::BufRead,
    line: String,
}

impl<'reader, 'token_types> Tokenizer<'reader, 'token_types> {
    pub fn new(reader: &'reader mut dyn std::io::BufRead, token_types: &'token_types [TokenType]) 
    -> Tokenizer<'reader, 'token_types> {
        Tokenizer{
            reader,
            token_types,
            line: String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct TokenNotMatchedError {}
impl error::Error for TokenNotMatchedError {}
impl fmt::Display for TokenNotMatchedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Token not matched");
    }
}

impl<'reader, 'token_types> Iterator for Tokenizer<'reader, 'token_types> {
    type Item = Result<Token<'token_types>, Box<dyn error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.line.is_empty(){
            if let Ok(0) = self.reader.read_line(&mut self.line) {
                return None;
            };
        }
       
        for length in (1..(self.line.chars().count() + 1)).rev() {
            let buffer: &String = &self.line.chars().take(length).collect();
            for token_type in self.token_types {
                if let Some(token) = token_type.match_token(buffer){
                    self.line = self.line.chars().skip(buffer.chars().count()).collect();
                    return Some(Ok(token));
                }
            }

        }

        return Some(Err(Box::new(TokenNotMatchedError {})))    
    }
}