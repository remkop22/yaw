use crate::common::Symbol;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Rule<T, NT> {
	lhs: NT,
	symbols: Vec<Symbol<T, NT>>,
}

impl<T, NT> Rule<T, NT>
where
	T: Copy,
	NT: Copy,
{
	pub fn new(lhs: NT, symbols: Vec<Symbol<T, NT>>) -> Self {
		Self {
			lhs,
			symbols,
		}
	}

	pub fn lhs(&self) -> NT {
		return self.lhs;
	}

	pub fn lhs_as_sym(&self) -> Symbol<T, NT> {
		Symbol::NonTerminal(self.lhs)
	}

	pub fn symbols(&self) -> &Vec<Symbol<T, NT>> {
		return &self.symbols;
	}
}

impl<'a, T, NT> IntoIterator for &'a Rule<T, NT> {
	type Item = &'a Symbol<T, NT>;
	type IntoIter = std::slice::Iter<'a, Symbol<T, NT>>;

	fn into_iter(self) -> Self::IntoIter {
		return self.symbols.iter();
	}
}
