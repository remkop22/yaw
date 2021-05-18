
use super::{ Symbol, Rule, NonTerminal };
use std::collections::HashSet;

pub trait Analyser<'r> {
   
    fn get_rules(& self) -> &'r Vec<Rule>;

    fn find_rules_by_lhs(&self, lhs: &NonTerminal) -> Vec<&'r Rule> {
        let mut rules = Vec::new();
        for rule in self.get_rules() {
            if rule.get_lhs() == lhs {
                rules.push(rule);
            }
        }
        
        return rules;
    }

    fn unique_symbols(&self) -> (HashSet<Symbol>, HashSet<Symbol>) {
        let mut terminals = HashSet::new();
        let mut nonterminals = HashSet::new();
        for rule in self.get_rules() {
            for sym in rule.get_symbols() {
                if sym.is_terminal() {
                    terminals.insert(sym.clone());
                }else{
                    nonterminals.insert(sym.clone());
                }
            }

            nonterminals.insert(rule.get_lhs_as_sym());
        }

        return (terminals, nonterminals);
    } 


    
}