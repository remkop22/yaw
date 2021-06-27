
pub struct Token<T> {
	pub span: (usize, usize),
	pub kind: T,
}

pub trait Parser<T> {
	fn next_token(&mut self) -> Token<T>;
	fn action(&mut self, token: Token<T>);
	fn state(&self) -> usize;
	fn pop_state(&self);
	fn push_state(&self, state: usize);
	
	fn push_token(&self, token: Token<T>);
	fn push_rule(&self, rule_index: usize);

	fn parse(&mut self) {
		loop {
			let token = self.next_token();
			self.action(token);
		}
	}

	fn shift(&mut self, shift_state: usize, token: Token<T>) {
		self.push_token(token);
		self.push_state(shift_state)
	}

	fn reduce(&mut self, rule_index: usize){
		self.push_rule(rule_index);
		self.pop_state();
		self.goto(rule_index);
	}

	fn goto(&mut self, rule_index: usize); 

	fn error(&mut self) {}
}
