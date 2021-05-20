
use super::{Rule, Symbol, NonTerminal};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Action {
    Shift(usize),
    Reduce(Rule),
    Accept,
    Error
}

pub struct Conflict {
    pub first_action: Action,
    pub second_action: Action,
    pub symbol: Symbol,
    pub state: usize
}

pub struct Table{
    action: HashMap<usize, HashMap<Symbol, Action>>,
    goto: HashMap<usize, HashMap<NonTerminal, usize>>,
    conflicts: Vec<Conflict>
}

impl Table {

    pub fn new() -> Self {
        return Self { 
            action: HashMap::new(), 
            goto: HashMap::new(),
            conflicts: Vec::new() 
        };
    }

    pub fn insert_action(&mut self, index: usize, terminal: Symbol, action: Action) {
        if let Some(row) = self.action.get_mut(&index) {
            if row.contains_key(&terminal){
                println!("confict");
                self.conflicts.push(Conflict {
                    first_action: row[&terminal].clone(),
                    second_action: action.clone(),
                    state: index,
                    symbol: terminal.clone()
                });
            }
           
            row.insert(terminal, action);
        }else {
            let mut row = HashMap::new();    
            row.insert(terminal, action);
            self.action.insert(index, row);
        }

        println!("{:?}", self.action.keys())
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

    pub fn actions(&self) -> &HashMap<usize, HashMap<Symbol, Action>> {
        return &self.action;
    }

    pub fn conflicts(&self) -> &Vec<Conflict> {
        return &self.conflicts;
    }
    
    pub fn gotos(&self) -> &HashMap<usize, HashMap<NonTerminal, usize>> {
        return &self.goto;
    }

}