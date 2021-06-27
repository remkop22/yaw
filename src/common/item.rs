use crate::common::{Rule, Symbol};
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Item<'r, T, NT> {
	rule: &'r Rule<T, NT>,
	index: usize,
	look_ahead: T,
}

impl<'r, T, NT> Item<'r, T, NT>
where
	T: Copy,
	NT: Copy,
{
	pub fn new(rule: &'r Rule<T, NT>, index: usize, look_ahead: T) -> Self {
		return Self {
			rule,
			index,
			look_ahead,
		};
	}

	pub fn advance(&self) -> Option<Self> {
		if self.is_active() {
			return Some(Self::new(self.rule, self.index + 1, self.look_ahead));
		} else {
			return None;
		}
	}

	pub fn rule(&self) -> &'r Rule<T, NT> {
		self.rule
	}

	pub fn active_symbol(&self) -> Option<Symbol<T, NT>> {
		return self.rule.symbols().get(self.index).map(|s| *s);
	}

	pub fn is_active(&self) -> bool {
		return self.index < self.rule.symbols().len();
	}

	pub fn following_active(&self) -> Symbol<T, NT> {
		if self.index + 1 < self.rule.symbols().len() {
			return self.rule.symbols()[self.index + 1];
		} else {
			return Symbol::Terminal(self.look_ahead);
		}
	}

	pub fn look_ahead(&self) -> T {
		return self.look_ahead;
	}
}
