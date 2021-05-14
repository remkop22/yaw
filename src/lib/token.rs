
use regex::Regex;

pub trait Matcher {
    fn is_match(&self, text: &str) -> bool;
}

pub struct TokenType{
    name: String,
    matcher: Box<dyn Matcher>
}

impl Matcher for Regex {
    fn is_match(&self, text: &str) -> bool {
        return self.is_match(text)
    }
}

impl Matcher for Vec<&str> {
    fn is_match(&self, text: &str) -> bool {
        for item in self{
            if *item == text {
                return true;
            }
        }

        return false;
    }
}

impl<'a> TokenType {
    pub fn new(name: String, matcher: impl Matcher + 'static) -> TokenType {
        return TokenType{
            name,
            matcher: Box::new(matcher)
        }
    }

    pub fn match_token<'b>(self: &'a Self, text: &'b str) -> Option<Token> {
        if self.matcher.is_match(text){
            return Some(Token::new(String::from(text), self));
        }else{
            return None;
        }
    }

    pub fn get_name(&self) -> &str {
        return &self.name[..]
    }
}

pub struct Token<'kind>{
    value: String,
    kind: &'kind TokenType
}

impl<'kind> Token<'kind> {
    pub fn new(value: String, kind: &'kind TokenType) -> Token<'kind> {
        return Token{ value, kind };
    }
    pub fn get_value(&self) -> &str {
        return &self.value[..];
    }
    pub fn get_kind(&self) -> &TokenType {
        return self.kind;
    }

}
