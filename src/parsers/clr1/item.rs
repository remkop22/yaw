
use crate::parsers::common::{ Symbol, Rule, Item };

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct LR1Item<'r, T, NT>{
    rule: &'r Rule<T, NT>,
    index: usize,
    look_ahead: Symbol<T, NT>
}

impl<'r, T, NT> LR1Item<'r, T, NT> {

    pub fn new(rule: &'r Rule<T, NT>, index: usize, look_ahead: Symbol<T, NT>) -> Self {
        return Self {rule, index, look_ahead}
    }

    pub fn following_active(&self) -> &Symbol<T, NT> {
        if self.index + 1 < self.rule.symbols().len() {
            return &self.rule.symbols()[self.index + 1];
        } else {
            return self.look_ahead();
        }
    }

    pub fn look_ahead(&self) -> &Symbol<T, NT> {
        return &self.look_ahead;
    }
    
}

impl<'r, T, NT> Item<T, NT> for LR1Item<'r, T, NT> {

    fn index(&self) -> usize {
        return self.index;
    }

    fn rule(&self) -> &Rule<T, NT> {
        return self.rule;
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

}

