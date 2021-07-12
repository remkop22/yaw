use crate::common::{Rule, Symbol};
use std::collections::HashMap;
use std::hash::Hash;

pub type StateIndex = usize;
pub type ActionTable<Term, NonTerm> = HashMap<StateIndex, HashMap<Term, Action<Term, NonTerm>>>;
pub type GotoTable<NonTerm> = HashMap<StateIndex, HashMap<NonTerm, StateIndex>>;

#[derive(Clone, Debug)]
pub enum Action<T, NT> {
    Shift(StateIndex),
    Reduce(Rule<T, NT>),
    Accept,
    Error,
}

#[derive(Debug)]
pub struct Conflict<T, NT> {
    pub first_action: Action<T, NT>,
    pub second_action: Action<T, NT>,
    pub symbol: Symbol<T, NT>,
    pub state: StateIndex,
}

pub struct Table<T, NT> {
    action: ActionTable<T, NT>,
    goto: GotoTable<NT>,
    conflicts: Vec<Conflict<T, NT>>,
}

impl<Term, NonTerm> Table<Term, NonTerm>
where
    Term: Eq + Hash + Copy,
    NonTerm: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            action: HashMap::new(),
            goto: HashMap::new(),
            conflicts: Vec::new(),
        }
    }

    pub fn insert_action(
        &mut self,
        index: StateIndex,
        terminal: Term,
        action: Action<Term, NonTerm>,
    ) {
        if let Some(row) = self.action.get_mut(&index) {
            if row.contains_key(&terminal) {
                self.conflicts.push(Conflict {
                    first_action: row[&terminal].clone(),
                    second_action: action.clone(),
                    state: index,
                    symbol: Symbol::Terminal(terminal),
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
        &self.action
    }

    pub fn conflicts(&self) -> &Vec<Conflict<Term, NonTerm>> {
        &self.conflicts
    }

    pub fn gotos(&self) -> &GotoTable<NonTerm> {
        &self.goto
    }
}
