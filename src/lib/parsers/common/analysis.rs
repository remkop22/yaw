
use super::{ Symbol, Rule, NonTerminal };

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


    
}