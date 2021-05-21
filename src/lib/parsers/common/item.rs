use super::{ Symbol, Rule };

use std::hash::Hash;

pub trait Item: Sized + Eq + Hash + Clone {

    fn rule(&self) -> &Rule;

    fn index(&self) -> usize;
    fn set_index(&mut self, index: usize);

    fn advance(&self) -> Option<Self> {
        if self.is_active() { 
            let mut cp = self.clone();
            cp.set_index(cp.index() + 1);
            return Some(cp);
        } else {
            return None;
        }
    }

    fn active_symbol(&self) -> Option<&Symbol> { 
        return self.rule().symbols().get(self.index());
    }
        
    fn is_active(&self) -> bool {
        return self.index() < self.rule().symbols().len();
    }



}