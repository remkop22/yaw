

use super::{ Rule, Item, Table, Symbol, ItemSet };

use std::cmp::Eq;
use std::hash::Hash;

pub fn generate_table<Terminal: Eq + Hash + Clone, NonTerminal: Eq + Hash + Clone>(
    rules: &Vec<Rule<Terminal, NonTerminal>>, 
    start: NonTerminal
) -> Table<Terminal, NonTerminal> {
    let mut generator = Generator::new(rules, start);
    generator.generate_table();
    return generator.table;
} 

pub enum Action<Terminal, NonTerminal> {
    Shift(usize),
    Reduce(Rule<Terminal, NonTerminal>),
    Accept
}

pub struct Generator<'rules, Terminal, NonTerminal> {
    rules: &'rules Vec<Rule<Terminal, NonTerminal>>,
    table: Table<Terminal, NonTerminal>,
    start: NonTerminal,
    states: Vec<ItemSet<'rules, Terminal, NonTerminal>>,
    state_index: usize
}

impl<'rules, Terminal: Eq + Hash + Clone, NonTerminal: Eq + Hash + Clone> Generator<'rules, Terminal, NonTerminal>{

    fn new(rules: &'rules Vec<Rule<Terminal, NonTerminal>>, start: NonTerminal) -> Self {

        return Self{ 
            rules, 
            table: Table::new(),
            states: Vec::new(),
            state_index: 0,
            start
        };
    }

    fn generate_table<'a>(&'a mut self) {

        let mut start_rule = None;
        for rule in self.rules {
            if rule.get_lhs() == &self.start {
                start_rule = Some(rule);
            }
        }

        let start_rule = &start_rule.expect("No rule found with start symbol as lhs");
        let start_item = Item::new(start_rule, 0, Symbol::EndOfTokenStream);
        let mut start_set = ItemSet::new();
        start_set.insert_kernel(start_item);
        self.states.push(start_set);
        
        while self.state_index < self.states.len() {
            //self.close();
            self.goto_all();
            self.reduce_all();
            self.state_index += 1;
        }

    }

    fn find_rules_by_lhs(&self, lhs: &NonTerminal) -> Vec<&Rule<Terminal, NonTerminal>> {
        let mut rules = Vec::new();
        for rule in self.rules {
            if rule.get_lhs() == lhs {
                rules.push(rule);
            }
        }
        
        return rules;
    }

    fn find_state_index(&self, set: &ItemSet<'rules, Terminal, NonTerminal>) -> Option<usize>{
        for (i, match_set) in self.states.iter().enumerate(){
            if set.is_same_kernel(match_set){
                return Some(i)
            }
        }

        return None;
    }

    fn goto_all(&mut self){
        for sym in self.states[self.state_index].get_active_symbols(){
            self.goto(sym);
        }
    }

    fn reduce_all(&mut self){
        for item in self.states[self.state_index].all() {
            // If item is at the end it means a reduce action of it's rule is appropriate.
            if !item.is_active() {
                // If the rule to reduce is the start rule we should insert an 'Accept' action,
                // if not we insert a normal reduce action.
                if item.get_lhs() == &self.start {
                    self.table.insert_action(self.state_index, item.get_look_ahead().clone(), Action::Accept);
                } else {
                    let reduce = Action::Reduce(item.get_rule().clone());
                    self.table.insert_action(self.state_index, item.get_look_ahead().clone(), reduce)
                }
            } 
        }
    }

    fn goto(&mut self, sym: Symbol<Terminal, NonTerminal>){
        let mut new_set = ItemSet::new();
        
        // Create a new set of items from the current item set,
        // where each item has an active symbol equal to sym.
        for item in self.states[self.state_index].all() {
            if let Some(active_sym) = item.get_active_symbol() {
                if active_sym == &sym {
                    new_set.insert_kernel(item.goto().unwrap());
                }
            }
        }

        // Check if item set with the same kernel already exists,
        // if so use that index for the transition in the action table,
        // else insert it into the states and use that index.
        let index = self.find_state_index(&new_set).unwrap_or_else( || { 
            self.states.push(new_set);
            return self.states.len();
        });

        match sym.clone() { 
            Symbol::NonTerminal(non_term) => self.table.insert_goto(self.state_index, non_term.clone(), index),
            Symbol::Terminal(term, value) => self.table.insert_action(index, Symbol::Terminal(term, value), Action::Shift(self.state_index)),
            Symbol::EndOfTokenStream => panic!("Do not use 'EndOfTokenStream' symbol explicitly in grammar")
        }
    }



}