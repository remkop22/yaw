
use super::{Rule, Symbol};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Item<'rule, Terminal, NonTerminal>{
    rule: &'rule Rule<Terminal, NonTerminal>,
    index: usize,
    look_ahead: Symbol<Terminal, NonTerminal>
}

impl<'rule, Terminal: Clone, NonTerminal: Clone> Item<'rule, Terminal, NonTerminal> {

    pub fn new(
        rule: &'rule Rule<Terminal, NonTerminal>, 
        index: usize, 
        look_ahead: Symbol<Terminal, NonTerminal>
    ) -> Self {
        return Self {
            rule,
            index,
            look_ahead
        }
    }

    pub fn get_lhs(&self) -> &NonTerminal {
        return self.rule.get_lhs();
    }

    pub fn get_rule(&self) -> &Rule<Terminal, NonTerminal> {
        return self.rule;
    }

    pub fn is_active(&self) -> bool {
        return self.index < self.rule.get_symbols().len();
    }

    pub fn get_following_active(&self) -> &Symbol<Terminal, NonTerminal> {
        if self.index + 1 < self.rule.get_symbols().len() {
            return &self.rule.get_symbols()[self.index + 1];
        } else {
            return self.get_look_ahead();
        }
    }

    pub fn get_look_ahead(&self) -> &Symbol<Terminal, NonTerminal> {
        return &self.look_ahead;
    }

    pub fn get_active_symbol(&self) -> Option<&Symbol<Terminal, NonTerminal>> {
        return self.rule.get_symbols().get(self.index);
    }

    pub fn goto(self: &Self) -> Option<Item<'rule, Terminal, NonTerminal>> {
        if self.is_active() {
            return Some(Item::new(self.rule, self.index + 1, self.look_ahead.clone()));
        } else {
            return None;
        }
    }

}

