

use super::Action;
use super::super::grammar::Symbol;

use std::collections::HashMap;
use std::hash::Hash;

pub enum Conflict {
    Warn,
    Panic,
    Ignore
}

pub struct Table<Terminal, NonTerminal> {
    action: HashMap<usize, HashMap<Symbol<Terminal, NonTerminal>, Action<Terminal, NonTerminal>>>,
    goto: HashMap<usize, HashMap<NonTerminal, usize>>,
    conflict_action: Conflict 
}

impl<Terminal: Eq + Hash, NonTerminal: Eq + Hash> Table<Terminal, NonTerminal> {
    pub fn new(conflict_action: Conflict) -> Self {
        return Self { 
            action: HashMap::new(), 
            goto: HashMap::new(),
            conflict_action
        };
    }

    pub fn insert_action(&mut self, index: usize, terminal: Symbol<Terminal, NonTerminal>, action: Action<Terminal, NonTerminal>) {
        if let Some(row) = self.action.get_mut(&index) {
            if row.contains_key(&terminal){
                match self.conflict_action {
                    Conflict::Warn => eprintln!("Conflict found!"),
                    Conflict::Panic => panic!("Conflict found!"),
                    Conflict::Ignore => {}
                }
            } 
            
            row.insert(terminal, action);
        }else {
            let mut row = HashMap::new();    
            row.insert(terminal, action);
            self.action.insert(index, row);
        }
    }

    pub fn insert_goto(&mut self, index: usize, lhs: NonTerminal, to_state: usize){
        if let Some(row) = self.goto.get_mut(&index) {
            row.insert(lhs, to_state);
        }else {
            let mut row = HashMap::new();    
            row.insert(lhs, to_state);
            self.goto.insert(index, row);
        }
    }

    pub fn get_actions(&self) -> &HashMap<usize, HashMap<Symbol<Terminal, NonTerminal>, Action<Terminal, NonTerminal>>> {
        return &self.action;
    }
    
    pub fn get_gotos(&self) -> &HashMap<usize, HashMap<NonTerminal, usize>> {
        return &self.goto;
    }

}