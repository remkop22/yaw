
use super::super::common::{ Rule, Symbol, Action, Table, Terminal, EOF, Analyser, LRAnalyser, ItemSet, Item };
use super::item::LR1Item;

use std::collections::{HashSet, HashMap}; 

pub struct CLR1Analyser<'r> {
    rules: &'r Vec<Rule>,
    table: Table,
    start: Rule,
    states: Vec<ItemSet<LR1Item<'r>>>,
    first_set: HashMap<Symbol, HashSet<Symbol>>,
}

impl<'r> CLR1Analyser<'r>{

    pub fn new(rules: &'r Vec<Rule>, start: &Rule) -> Self {

        let start_item = LR1Item::new(start, 0, Symbol::Terminal(Terminal::new(EOF.to_string(), true)));
        let start_set = ItemSet::from_kernel(vec![start_item]);
 
        let mut analyser = Self{ 
            rules, 
            start: start.clone(),
            table: Table::new(),
            states: vec![start_set],
            first_set: HashMap::new()
        };

        analyser.first_set = analyser.generate_first_set();
        analyser.table = analyser.generate_table();

        return analyser;
    } 

}

impl<'r> Analyser<'r> for CLR1Analyser<'r> {

    fn get_rules(&self) -> &'r Vec<Rule> {
        return self.rules;
    }

}

impl<'r> LRAnalyser<'r, LR1Item<'r>> for CLR1Analyser<'r> {

    fn get_states(&self) -> &Vec<ItemSet<LR1Item<'r>>> {
        return &self.states;
    }

    fn get_states_mut(&mut self) -> &mut Vec<ItemSet<LR1Item<'r>>> {
        return &mut self.states;
    }

    fn get_table(&self) -> &Table{
        return &self.table;
    }

    fn get_table_mut(&mut self) -> &mut Table{
        return &mut self.table;
    }


    fn reduce_item(&self, item: &LR1Item<'r>) -> (Symbol, Action) {
        // If the rule to reduce is the start rule we should insert an 'Accept' action,
        // if not we insert a normal reduce action.
        if item.get_lhs() == self.start.get_lhs() {
            (item.get_look_ahead().clone(), Action::Accept)
        } else {
            (item.get_look_ahead().clone(), Action::Reduce(item.get_rule().clone()))
        }
    }

    fn close_item(&self, item: &LR1Item) -> HashSet<LR1Item<'r>>{
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

                for rule in self.find_rules_by_lhs(&lhs) {
                    for look_ahead in look_aheads {
                        let item = LR1Item::new(rule, 0, look_ahead.clone()); 
                        result.insert(item);
                    }
                }
            }
        }

        return result;
    }
}