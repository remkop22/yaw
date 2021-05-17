
use super::token::{ TokenPattern, Token };

use std::error;
use std::fmt;
use std::io;

pub struct Tokenizer<'patterns, 'source, TokenType>{
    stopped: bool,
    buffer: String,
    patterns: &'patterns [TokenPattern<'patterns, TokenType>],
    source: & 'source mut dyn io::BufRead
}

impl<'patterns, 'source, TokenType> Tokenizer<'patterns, 'source, TokenType>{
    pub fn new(source: &'source mut dyn io::BufRead, patterns: &'patterns [TokenPattern<TokenType>]) -> Self{
        return Self {
            stopped: false,
            buffer: String::from(""),
            patterns,
            source
        };
    }

}

impl<'a, 'b, TokenType: Copy> Iterator for Tokenizer<'a, 'b, TokenType> {
    type Item = Result<Token<TokenType>, Box<dyn error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
       
        let mut next_buffer = String::from("");
        if self.buffer.is_empty() {
            if let Ok(0) = self.source.read_line(&mut self.buffer){
                return None; 
            }

       }
       
        loop {

            for pattern in self.patterns {
                if let Some(token) = pattern.match_token(&self.buffer[..]) {
                    self.buffer = next_buffer.chars().rev().collect();
                    return Some(Ok(token));
                }
            }

            match self.buffer.pop() {
                Some(c) => { next_buffer.push(c) }
                None => { 
                    self.stopped = true;
                    return Some(Err(Box::new(TokenNotMatchedError { buffer: next_buffer.clone() }))) 
                }
            }

        }

    }    
}

#[derive(Debug)]
pub struct TokenNotMatchedError {
    buffer: String
}
impl error::Error for TokenNotMatchedError {}
impl fmt::Display for TokenNotMatchedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Token '{}' not matched", self.buffer);
    }
}

