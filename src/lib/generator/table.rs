
use super::{ Action, Symbol };
use std::collections::HashMap;
use std::hash::Hash;

pub struct Table<Terminal, NonTerminal> {
    action: Vec<HashMap<Symbol<Terminal, NonTerminal>, Action<Terminal, NonTerminal>>>,
    goto: Vec<HashMap<NonTerminal, usize>>
}

impl<Terminal: Eq + Hash, NonTerminal: Eq + Hash> Table<Terminal, NonTerminal> {
    pub fn new() -> Self {
        return Self { action: Vec::new(), goto: Vec::new() };
    }

    pub fn insert_action(&mut self, index: usize, terminal: Symbol<Terminal, NonTerminal>, action: Action<Terminal, NonTerminal>) {
        if index < self.action.len() {
            self.action[index].insert(terminal, action);
        }else {
            let mut action_row = HashMap::new();
            action_row.insert(terminal, action);
            self.action.push(action_row);
        }
    }

    pub fn insert_goto(&mut self, index: usize, lhs: NonTerminal, to_state: usize){
        if index < self.goto.len() {
            self.goto[index].insert(lhs, to_state);
        } else {
            let mut goto_row = HashMap::new();
            goto_row.insert(lhs, to_state);
            self.goto.push(goto_row);
        }
    }
}