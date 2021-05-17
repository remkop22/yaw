

use super::super::grammar::{Rule, Symbol};
use super::super::grammar::itemset::ItemSet;
use super::super::grammar::item::Item;

use super::{Table, Action, Conflict};

use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};

pub fn generate_table<Terminal: Eq + Hash + Clone + Debug, NonTerminal: Eq + Hash + Clone + Debug>(
    rules: &Vec<Rule<Terminal, NonTerminal>>, 
    start: NonTerminal,
    conflict_action: Conflict
) -> Table<Terminal, NonTerminal> {
    let mut generator = Generator::new(rules, start, conflict_action);
    generator.generate_table();
    return generator.table;
} 

pub struct Generator<'rules, Terminal, NonTerminal> {
    rules: &'rules Vec<Rule<Terminal, NonTerminal>>,
    table: Table<Terminal, NonTerminal>,
    start: NonTerminal,
    states: Vec<ItemSet<'rules, Terminal, NonTerminal>>,
    state_index: usize,
    first_set: HashMap<Symbol<Terminal, NonTerminal>, HashSet<Symbol<Terminal, NonTerminal>>>,
}

impl<'rules, Terminal: Eq + Hash + Clone + Debug, NonTerminal: Eq + Hash + Clone + Debug> Generator<'rules, Terminal, NonTerminal>{

    fn new(rules: &'rules Vec<Rule<Terminal, NonTerminal>>, start: NonTerminal, conflict_action: Conflict) -> Self {
        return Self{ 
            rules, 
            start,
            table: Table::new(conflict_action),
            states: Vec::new(),
            state_index: 0,
            first_set: HashMap::new()
        };
    }

    fn generate_first_set(&mut self) {
        let (terms, nonterms) = self.unique_symbols();
        let mut first_set = HashMap::new();
        let mut empty: HashSet<Symbol<_, _>> = HashSet::new();
        
        for nonterm in nonterms {
            first_set.insert(nonterm, HashSet::new());
        }
       
        for term in terms {
            let mut set = HashSet::new();
            set.insert(term.clone());
            first_set.insert(term, set);
        }

        loop {
            let mut updated = false;

            for rule in self.rules {
                let mut lhs_set = first_set[&rule.get_lhs_as_sym()].clone();
                let mut all_empty = false;
               
                for sym in rule.get_symbols() {
                    updated = first_set[sym].is_subset(&lhs_set);
                    lhs_set.extend(first_set[sym].clone());
                    if !empty.contains(sym) {
                        all_empty = true;
                        break;
                    }
                }

                if !all_empty {
                    updated = empty.insert(rule.get_lhs_as_sym());
                }

                first_set.insert(rule.get_lhs_as_sym(), lhs_set);
            }

            if !updated {
                break;
            }
        }
        
        self.first_set = first_set;

    }

    fn unique_symbols(&self) -> (HashSet<Symbol<Terminal, NonTerminal>>, HashSet<Symbol<Terminal, NonTerminal>>) {
        let mut terminals = HashSet::new();
        let mut nonterminals = HashSet::new();
        for rule in self.rules {
            for sym in rule.get_symbols() {
                match sym {
                    Symbol::Terminal(term, value) => { terminals.insert(Symbol::Terminal(term.clone(), *value)); },
                    Symbol::NonTerminal(nonterm) => { nonterminals.insert(Symbol::NonTerminal(nonterm.clone())); },
                    Symbol::EndOfTokenStream => {}
                };
            }

            nonterminals.insert(rule.get_lhs_as_sym());
        }

        return (terminals, nonterminals);
    } 

    fn generate_table(&mut self) {
       
        self.generate_first_set();

        let start_rules = self.find_rules_by_lhs(&self.start);
        let start_rule = start_rules.first().expect("No rule with lhs of start symbol found");
        let start_item = Item::new(&start_rule, 0, Symbol::EndOfTokenStream);
        let start_set = ItemSet::from_kernel(vec![start_item]);
        self.states.push(start_set);
        
        while self.state_index < self.states.len() {
            self.close();
            self.goto_all();
            self.reduce_all();
            self.state_index += 1;
        }

    }

    fn find_rules_by_lhs(&self, lhs: &NonTerminal) -> Vec<&'rules Rule<Terminal, NonTerminal>> {
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

    fn close(&mut self){
        let mut queue = HashSet::new();
        let mut already_closed = HashSet::new();

        // Fill the queue with initial closure of the kernel.
        queue.extend(self.close_set(self.states[self.state_index].get_kernel(), &mut already_closed));
        
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
        self.states[self.state_index].extend_closure(queue);

    }

    fn close_set(&self, 
        set: &HashSet<Item<Terminal, NonTerminal>>, 
        already_closed: &mut HashSet<Symbol<Terminal, NonTerminal>>
    ) -> HashSet<Item<'rules, Terminal, NonTerminal>> {
        // This function only shallowly closes an item. the resulting set may return unclosed items.

        let mut result = HashSet::new();
        // If an item in the set is active we should close on the active symbol,
        // if not it should be ingored.
        for item in set {
            if let Some(sym) = item.get_active_symbol() {
                // 'already_closed' keeps a record of all previously closed symbols,
                // if the active symbol of 'item' is already in this set it can be assumed that the
                // resulting closure is already present somewhere. Without this, recursion problems would occure.
                if !already_closed.contains(sym) {
                    result.extend(self.close_item(item));
                    already_closed.insert(sym.clone());
                }
            }
        }

        return result;
    }

    fn close_item(&self, item: &Item<Terminal, NonTerminal>) -> HashSet<Item<'rules, Terminal, NonTerminal>>{
        // This function only shallowly closes an item. the resulting set may return unclosed items.
        let mut result = HashSet::new();

        // In order to close an item, it must be active and it's active symbol must be a reference to other rules.
        // If this is not the case the resulting closure consists of an empty set.
        if let Some(sym) = item.get_active_symbol() {
            if let Symbol::NonTerminal(lhs) = sym {
                // Find the first set of the symbol following the active symbol, 
                // if the item is not active or there is no symbol following the active symbol,
                // use the the look_ahead of this item.
                let look_aheads = self.first_set.get(item.get_following_active())
                    .expect(&format!("Fatal error, first set does not contain {:?}", item.get_following_active())); 

                for rule in self.find_rules_by_lhs(lhs) {
                    for look_ahead in look_aheads {
                        let item = Item::new(rule, 0, look_ahead.clone()); 
                        result.insert(item);
                    }
                }
            }
        }

        return result;
    }

}