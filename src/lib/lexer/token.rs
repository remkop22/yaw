

pub trait Pattern {
    fn is_match(self: &Self, text: &str) -> bool;
}

impl Pattern for regex::Regex {
    fn is_match(self: &Self, text: &str) -> bool {
        return regex::Regex::is_match(self, text);
    }
}

impl Pattern for Vec<&str> {
    fn is_match(self: &Self, text: &str) -> bool {
        return self.contains(&text);
    }
}

pub struct TokenPattern<'pattern,T> {
    kind: T,
    pattern: Box<dyn Pattern + 'pattern> 
}

impl<'pattern, T> TokenPattern<'pattern, T> {

    pub fn new<P: Pattern + 'pattern>(kind: T, pattern: P) -> Self {
        return TokenPattern { kind, pattern: Box::new(pattern) }
    }

    pub fn match_token(self: &Self, text: &str) -> Option<Token<T>> where T: Copy{
        if self.pattern.is_match(text) {
            return Some(Token::new(self.kind, String::from(text)));
        } else {
            return None;
        }
    }
}

pub struct Token<T> {
    kind: T,
    value: String
}

impl<T> Token<T> {

    pub fn new(kind: T, value: String) -> Self {
        return Self { kind, value };
    }

    pub fn get_kind(self: &Self) -> &T {
        return &self.kind;
    }

    pub fn get_value(self: &Self) -> &str {
        return &self.value[..];
    }
}