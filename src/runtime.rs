pub struct Token<T> {
    pub span: (usize, usize),
    pub kind: T,
}

pub struct ParseState<I, T> {
    tokens: I,
    token_stack: Vec<Token<T>>,
    rule_stack: Vec<usize>,
    state_stack: Vec<usize>,
}
impl<I, T> ParseState<I, T> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            token_stack: Vec::new(),
            rule_stack: Vec::new(),
            state_stack: vec![0],
        }
    }
}
pub trait Parse<I, T>
where
    I: Iterator<Item = Token<T>>,
{
    fn parse_state(&self) -> &ParseState<I, T>;
    fn parse_state_mut(&mut self) -> &mut ParseState<I, T>;

    fn next_token(&mut self) -> Option<Token<T>> {
        self.parse_state_mut().tokens.next()
    }

    fn push_token(&mut self, token: Token<T>) {
        self.parse_state_mut().token_stack.push(token);
    }

    fn push_state(&mut self, state: usize) {
        self.parse_state_mut().state_stack.push(state);
    }

    fn pop_state(&mut self) {
        self.parse_state_mut().state_stack.pop();
    }

    fn push_rule(&mut self, rule: usize) {
        self.parse_state_mut().rule_stack.push(rule);
    }

    fn state(&self) -> usize {
        *self
            .parse_state()
            .state_stack
            .last()
            .expect("empty parse state")
    }

    fn action(&mut self, token: Token<T>) -> Option<Result<(), ()>>;

    fn parse(&mut self) -> Result<(), ()> {
        loop {
            if let Some(token) = self.parse_state_mut().tokens.next() {
                match self.action(token) {
                    Some(res) => break res,
                    None => {}
                }
            } else {
                panic!("unhandled EOF")
            }
        }
    }

    fn shift(&mut self, shift_state: usize, token: Token<T>) {
        self.pop_state();
        self.push_token(token);
        self.push_state(shift_state)
    }

    fn reduce(&mut self, rule_index: usize) {
        self.push_rule(rule_index);
        self.pop_state();
        self.goto(rule_index);
    }

    fn goto(&mut self, rule_index: usize);

    fn error(&self) -> Result<(), ()> {
        Err(())
    }

    fn accept(&self) -> Result<(), ()> {
        Ok(())
    }
}
