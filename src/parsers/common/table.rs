use super::{Rule, Symbol};
use std::collections::HashMap;
use std::hash::Hash;

pub type StateIndex = usize;
pub type ActionTable<Term, NonTerm> = HashMap<StateIndex, HashMap<Symbol<Term, NonTerm>, Action<Term, NonTerm>>>;
pub type GotoTable<NonTerm> = HashMap<StateIndex, HashMap<NonTerm, StateIndex>>;

#[derive(Clone)]
pub enum Action<Term, NonTerm> {
	Shift(StateIndex),
	Reduce(Rule<Term, NonTerm>),
	Accept,
	Error,
}

pub struct Conflict<Term, NonTerm> {
	pub first_action: Action<Term, NonTerm>,
	pub second_action: Action<Term, NonTerm>,
	pub symbol: Symbol<Term, NonTerm>,
	pub state: StateIndex,
}

pub struct Table<Term, NonTerm> {
	action: ActionTable<Term, NonTerm>,
	goto: GotoTable<NonTerm>,
	conflicts: Vec<Conflict<Term, NonTerm>>,
}

impl<Term, NonTerm> Table<Term, NonTerm>
where
	Term: Eq + Hash,
	NonTerm: Eq + Hash
{

	pub fn new() -> Self {
		return Self {
			action: HashMap::new(),
			goto: HashMap::new(),
			conflicts: Vec::new(),
		};
	}

	pub fn insert_action(
		&mut self,
		index: StateIndex,
		terminal: Symbol<Term, NonTerm>,
		action: Action<Term, NonTerm>,
	) {
		if let Some(row) = self.action.get_mut(&index) {
			if row.contains_key(&terminal) {
				self.conflicts.push(Conflict {
					first_action: row[&terminal],
					second_action: action,
					state: index,
					symbol: terminal,
				});
			}

			row.insert(terminal, action);
		} else {
			let mut row = HashMap::new();
			row.insert(terminal, action);
			self.action.insert(index, row);
		}
	}

	pub fn insert_goto(&mut self, index: StateIndex, lhs: NonTerm, to_state: StateIndex) {
		if let Some(row) = self.goto.get_mut(&index) {
			row.insert(lhs, to_state);
		} else {
			let mut row = HashMap::new();
			row.insert(lhs, to_state);
			self.goto.insert(index, row);
		}
	}

	pub fn actions(&self) -> &ActionTable<Term, NonTerm> {
		return &self.action;
	}

	pub fn conflicts(&self) -> &Vec<Conflict<Term, NonTerm>> {
		return &self.conflicts;
	}

	pub fn gotos(&self) -> &GotoTable<NonTerm> {
		return &self.goto;
	}
}
