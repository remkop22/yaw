use super::super::common::{Action, Analyser, Item, ItemSet, LRAnalyser, Rule, Symbol, Table};
use super::item::LR1Item;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct CLR1Analyser<'r, T, NT> {
	rules: &'r Vec<Rule<T, NT>>,
	table: Table<T, NT>,
	states: Vec<ItemSet<LR1Item<'r, T, NT>>>,
	first_set: HashMap<Symbol<T, NT>, HashSet<Symbol<T, NT>>>,
}

impl<'r, Term, NonTerm> CLR1Analyser<'r, Term, NonTerm>
where
	Term: 'static + Eq + Hash + Copy + std::fmt::Debug,
	NonTerm: 'static + Eq + Hash + Copy + std::fmt::Debug,
{
	pub fn new(rules: &'r Vec<Rule<Term, NonTerm>>, start: &'r Rule<Term, NonTerm>) -> Self {
		let start_item = LR1Item::new(start, 0, Symbol::<Term, NonTerm>::EOF);
		let start_set = ItemSet::from_kernel(vec![start_item]);

		let mut analyser = Self {
			rules,
			table: Table::new(),
			states: vec![start_set],
			first_set: HashMap::new(),
		};

		analyser.generate_table();
		analyser.first_set = analyser.first_set();

		return analyser;
	}
}

impl<'r, Term, NonTerm> Analyser<'r, Term, NonTerm> for CLR1Analyser<'r, Term, NonTerm> {
	fn first_set(&self) -> &HashMap<Symbol<Term, NonTerm>, HashSet<Symbol<Term, NonTerm>>> {
		return &self.first_set;
	}

	fn rules(&self) -> &'r Vec<Rule<Term, NonTerm>> {
		return self.rules;
	}
}

impl<'r, T, NT> LRAnalyser<'r, LR1Item<'r, T, NT>, T, NT> for CLR1Analyser<'r, T, NT>
where
	T: 'static + Eq + Hash + Copy + std::fmt::Debug,
	NT: 'static + Eq + Hash + Copy + std::fmt::Debug,
{
	fn states(&self) -> &Vec<ItemSet<LR1Item<'r, T, NT>>> {
		return &self.states;
	}

	fn states_mut(&mut self) -> &mut Vec<ItemSet<LR1Item<'r, T, NT>>> {
		return &mut self.states;
	}

	fn table(&self) -> &Table<T, NT> {
		return &self.table;
	}

	fn table_mut(&mut self) -> &mut Table<T, NT> {
		return &mut self.table;
	}

	fn reduce_item(&self, item: &LR1Item<'r, T, NT>) -> (Symbol<T, NT>, Action<T, NT>) {
		// If the rule to reduce is the start rule we should insert an 'Accept' action,
		// if not we insert a normal reduce action.
		if let Symbol::EOF = item.look_ahead() {
			(*item.look_ahead(), Action::Accept)
		} else {
			(*item.look_ahead(), Action::Reduce(*item.rule()))
		}
	}

	fn close_item(&self, item: &LR1Item<T, NT>) -> HashSet<LR1Item<'r, T, NT>> {
		// This function only shallowly closes an item. the resulting set may return unclosed items.
		let mut result = HashSet::new();

		// In order to close an item, it must be active and it's active symbol must be a reference to other rules.
		// If this is not the case the resulting closure consists of an empty set.
		if let Some(sym) = item.active_symbol() {
			if let Symbol::NonTerminal(lhs) = sym {
				// Find the first set of the symbol following the active symbol,
				// if the item is not active or there is no symbol following the active symbol,
				// use the the look_ahead of this item.
				let look_aheads = self.first_set.get(item.following_active()).expect(&format!(
					"Fatal error, first set does not contain {:?}",
					item.following_active()
				));

				for rule in self.find_rules_by_lhs(&lhs) {
					for look_ahead in look_aheads {
						let item = LR1Item::new(rule, 0, *look_ahead);
						result.insert(item);
					}
				}
			}
		}

		return result;
	}
}
