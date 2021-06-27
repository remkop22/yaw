use crate::common::{Item, Symbol};

use std::collections::HashSet;
use std::hash::Hash;

pub struct ItemSet<'r, T, NT> {
	kernel: HashSet<Item<'r, T, NT>>,
	closure: HashSet<Item<'r, T, NT>>,
}

impl<'r, T, NT> ItemSet<'r, T, NT>
where
	T: Hash + Eq + Copy,
	NT: Hash + Eq + Copy
{
	pub fn new() -> Self {
		return Self {
			kernel: HashSet::new(),
			closure: HashSet::new(),
		};
	}

	pub fn from_kernel(kernel_items: Vec<Item<'r, T, NT>>) -> Self {
		return Self {
			kernel: kernel_items.into_iter().collect(),
			closure: HashSet::new(),
		};
	}

	pub fn active_symbols(&self) -> HashSet<Symbol<T, NT>> {
		self.items().iter().filter_map(|s| s.active_symbol()).collect()
	}

	pub fn kernel(&self) -> &HashSet<Item<'r, T, NT>> {
		return &self.kernel;
	}

	pub fn extend_closure(&mut self, closure: HashSet<Item<'r, T, NT>>) {
		self.closure.extend(closure);
	}

	pub fn items(
		&self,
	) -> Vec<&Item<'r, T, NT>> {
		return self.kernel.union(&self.closure).collect();
	}

	pub fn insert_kernel(&mut self, item: Item<'r, T, NT>) -> bool {
		return self.kernel.insert(item);
	}

	pub fn kernel_equal(&self, other_set: &ItemSet<'r, T, NT>) -> bool {
		return self.kernel == other_set.kernel;
	}
}
