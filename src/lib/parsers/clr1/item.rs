
use crate::lib::parsers::common::{ Symbol, NonTerminal, Rule };

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Item<'r>{
    rule: &'r Rule,
    index: usize,
    look_ahead: Symbol
}

impl<'r> Item<'r> {

    pub fn new(rule: &'r Rule, index: usize, look_ahead: Symbol) -> Self {
        return Self {rule, index, look_ahead}
    }

    pub fn get_lhs(&self) -> &NonTerminal {
        return self.rule.get_lhs();
    }

    pub fn get_index(&self) -> usize {
        return self.index;
    }

    pub fn get_rule(&self) -> &Rule {
        return self.rule;
    }

    pub fn is_active(&self) -> bool {
        return self.index < self.rule.get_symbols().len();
    }

    pub fn get_following_active(&self) -> &Symbol {
        if self.index + 1 < self.rule.get_symbols().len() {
            return &self.rule.get_symbols()[self.index + 1];
        } else {
            return self.get_look_ahead();
        }
    }

    pub fn get_look_ahead(&self) -> &Symbol {
        return &self.look_ahead;
    }

    pub fn get_active_symbol(&self) -> Option<&Symbol> {
        return self.rule.get_symbols().get(self.index);
    }

    pub fn goto(self: &Self) -> Option<Item<'r>> {
        if self.is_active() {
            return Some(Item::new(self.rule, self.index + 1, self.look_ahead.clone()));
        } else {
            return None;
        }
    }

}

