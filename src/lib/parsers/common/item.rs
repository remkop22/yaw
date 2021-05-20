use super::{ Symbol, Rule, NonTerminal };

use std::hash::Hash;

pub trait Item: Sized + Eq + Hash + Clone {

    fn get_rule(&self) -> &Rule;

    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);

    fn advance(&self) -> Option<Self> {
        if self.is_active() { 
            let mut cp = self.clone();
            cp.set_index(cp.get_index() + 1);
            return Some(cp);
        } else {
            return None;
        }
    }

    fn get_lhs(&self) -> &NonTerminal {
        return self.get_rule().get_lhs();
    }

    fn get_active_symbol(&self) -> Option<&Symbol> { 
        return self.get_rule().get_symbols().get(self.get_index());
    }
        
    fn is_active(&self) -> bool {
        return self.get_index() < self.get_rule().get_symbols().len();
    }



}