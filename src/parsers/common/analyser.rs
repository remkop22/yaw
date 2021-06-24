use super::{Action, Item, ItemSet, Rule, Symbol, Table};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Analyser<'r, Term, NonTerm>
where
	NonTerm: 'static + Eq + Hash ,
	Term: 'static + Eq + Hash + Copy,
{
	fn rules(&self) -> &'r Vec<Rule<Term, NonTerm>>;
	fn first_set(&self) -> HashMap<Symbol<Term, NonTerm>, HashSet<Symbol<Term, NonTerm>>> {
		let mut first_set = HashMap::new();
		let (terms, nonterms) = self.unique_symbols();
		let mut empty = HashSet::new();

		for nonterm in nonterms {
			first_set.insert(nonterm, HashSet::new());
		}

		for term in terms {
			let mut set = HashSet::new();
			set.insert(term);
			first_set.insert(term, set);
		}

		loop {
			let mut updated = false;

			for rule in self.rules() {
				let mut lhs_set = first_set[&rule.lhs_as_sym()];
				let mut all_empty = false;

				for sym in rule.symbols() {
					updated = first_set[sym].is_subset(&lhs_set);
					lhs_set.extend(first_set[sym]);
					if !empty.contains(sym) {
						all_empty = true;
						break;
					}
				}

				if !all_empty {
					updated = empty.insert(rule.lhs_as_sym());
				}

				first_set.insert(rule.lhs_as_sym(), lhs_set);
			}

			if !updated {
				break;
			}
		}

		first_set

	}

	fn find_rules_by_lhs(&self, lhs: &NonTerm) -> Vec<&'r Rule<Term, NonTerm>> {
		let mut rules = Vec::new();
		for rule in self.rules() {
			if rule.lhs() == lhs {
				rules.push(rule);
			}
		}

		return rules;
	}

	fn unique_symbols(
		&self,
	) -> (
		HashSet<Symbol<Term, NonTerm>>,
		HashSet<Symbol<Term, NonTerm>>,
	) {
		let mut terminals = HashSet::new();
		let mut nonterminals = HashSet::new();
		for rule in self.rules() {
			for sym in rule.symbols() {
				if sym.is_terminal() {
					terminals.insert(*sym);
				} else {
					nonterminals.insert(*sym);
				}
			}

			nonterminals.insert(rule.lhs_as_sym());
		}

		return (terminals, nonterminals);
	}
}

pub trait LRAnalyser<'r, I, T, NT>: Analyser<'r, T, NT>
where
	I: Item<T, NT>,
	T: Eq + Hash,
	NT: Eq + Hash,
{
	fn states(&self) -> &Vec<ItemSet<I>>;
	fn states_mut(&mut self) -> &mut Vec<ItemSet<I>>;
	fn table(&self) -> &Table<T, NT>;
	fn table_mut(&mut self) -> &mut Table<T, NT>;

	fn close_item(&self, item: &I) -> HashSet<I>;
	fn reduce_item(&self, item: &I) -> (Symbol<T, NT>, Action<T, NT>);

	fn reduce_state(&mut self, index: usize) {
		let mut actions = Vec::new();
		for item in self.states()[index].all() {
			// If item is at the end it means a reduce action of it's rule is appropriate.
			if !item.is_active() {
				actions.push(self.reduce_item(item));
			}
		}

		for (symbol, action) in actions {
			self.table_mut().insert_action(index, symbol, action);
		}
	}

	fn generate_table(&mut self) {
		let mut index = 0;

		while index < self.states().len() {
			self.close_state(index);
			self.goto_state(index);
			self.reduce_state(index);
			index += 1;
		}
	}

	fn find_state_index(&self, set: &ItemSet<I>) -> Option<usize> {
		for (i, match_set) in self.states().iter().enumerate() {
			if set.is_same_kernel(match_set) {
				return Some(i);
			}
		}

		return None;
	}

	fn close_state(&mut self, index: usize) {
		let mut queue = HashSet::new();
		let mut already_closed = HashSet::new();

		// Fill the queue with initial closure of the kernel.
		queue.extend(self.close_set(self.states()[index].kernel(), &mut already_closed));

		let mut new_queue = HashSet::new();
		loop {
			// Fill the empty new_queue 'buffer' with the closure of the current queue.
			new_queue.extend(self.close_set(&queue, &mut already_closed));

			// If the above operation yields no new closures, it means that the current queue is fully closed,
			// if not it we should add the new items to the queue to be closed in the next loop.
			if new_queue.is_empty() {
				break;
			} else {
				queue.extend(new_queue.clone());
				new_queue.clear();
			}
		}

		// Add the new found fully closed items the closure of the current set.
		self.states_mut()[index].extend_closure(queue);
	}

	fn goto_state(&mut self, index: usize) {
		for sym in self.states()[index].active_symbols() {
			self.goto_symbol(index, sym);
		}
	}

	fn close_set(&self, set: &HashSet<I>, already_closed: &mut HashSet<Symbol<T, NT>>) -> HashSet<I> {
		// This function only shallowly closes an item. the resulting set may return unclosed items.

		let mut result = HashSet::new();
		// If an item in the set is active we should close on the active symbol,
		// if not it should be ingored.
		for item in set {
			if let Some(sym) = item.active_symbol() {
				// 'already_closed' keeps a record of all previously closed symbols,
				// if the active symbol of 'item' is already in this set it can be assumed that the
				// resulting closure is already present somewhere. Without this, recursion problems would occure.
				if !already_closed.contains(sym) {
					result.extend(self.close_item(item));
					already_closed.insert(*sym);
				}
			}
		}

		return result;
	}

	fn goto_symbol(&mut self, index: usize, sym: Symbol<T, NT>) {
		let mut new_set = ItemSet::new();

		// Create a new set of items from the current item set,
		// where each item has an active symbol equal to sym.
		for item in self.states()[index].all() {
			if let Some(active_sym) = item.active_symbol() {
				if active_sym == &sym {
					new_set.insert_kernel(item.advance().unwrap());
				}
			}
		}

		// Check if item set with the same kernel already exists,
		// if so use that index for the transition in the action table,
		// else insert it into the states and use that index.
		let to_state = self.find_state_index(&new_set).unwrap_or_else(|| {
			self.states_mut().push(new_set);
			return self.states().len();
		});

		match sym {
			Symbol::NonTerminal(non_term) => {
				self.table_mut().insert_goto(index, non_term, to_state)
			}
			term => self
				.table_mut()
				.insert_action(index, term, Action::Shift(to_state)),
		}
	}
}
