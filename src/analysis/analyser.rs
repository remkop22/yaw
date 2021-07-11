use crate::analysis::{Action, Table};
use crate::common::{Grammar, Item, ItemSet, Symbol, Terminal};

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Analyser<'g, T, NT> {
    table: Table<T, NT>,
    states: Vec<ItemSet<'g, T, NT>>,
    grammar: &'g Grammar<T, NT>,
    first_set: HashMap<Symbol<T, NT>, HashSet<T>>,
}

impl<'g, T, NT> Analyser<'g, T, NT>
where
    T: 'static + Terminal,
    NT: 'static + Eq + Hash + Copy + std::fmt::Debug,
{
    pub fn new(grammar: &'g Grammar<T, NT>) -> Self {
        let start_item = Item::new(grammar.start_rule(), 0, T::eof());
        let start_set = ItemSet::from_kernel(vec![start_item]);

        let mut analyser = Self {
            grammar,
            table: Table::new(),
            states: vec![start_set],
            first_set: grammar.first_set(),
        };

        analyser.generate_table();

        analyser
    }

    pub fn table(&self) -> &Table<T, NT> {
        &self.table
    }

    fn reduce_state(&mut self, index: usize) {
        let mut actions = Vec::new();
        for item in self.states[index].items() {
            // If item is at the end it means a reduce action of it's rule is appropriate.
            if !item.is_active() {
                actions.push(self.reduce_item(item));
            }
        }

        for (symbol, action) in actions {
            self.table.insert_action(index, symbol, action);
        }
    }

    fn generate_table(&mut self) {
        let mut index = 0;

        while index < self.states.len() {
            self.close_state(index);
            self.goto_state(index);
            self.reduce_state(index);
            index += 1;
        }
    }

    fn state_index(&self, set: &ItemSet<'g, T, NT>) -> Option<usize> {
        for (i, match_set) in self.states.iter().enumerate() {
            if set.kernel_equal(match_set) {
                return Some(i);
            }
        }

        None
    }

    fn close_state(&mut self, index: usize) {
        let mut queue = HashSet::new();
        let mut already_closed = HashSet::new();

        // Fill the queue with initial closure of the kernel.
        queue.extend(self.close_set(self.states[index].kernel(), &mut already_closed));

        let mut new_queue = HashSet::new();
        loop {
            // Fill the empty new_queue 'buffer' with the closure of the current queue.
            new_queue.extend(self.close_set(&queue, &mut already_closed));

            // If the above operation yields no new closures, it means that the current queue is fully closed,
            // if not it we should add the new items to the queue to be closed in the next loop.
            if new_queue.is_empty() {
                break;
            } else {
                queue.extend(&new_queue);
                new_queue.clear();
            }
        }

        // Add the new found fully closed items the closure of the current set.
        self.states[index].extend_closure(queue);
    }

    fn goto_state(&mut self, index: usize) {
        for sym in self.states[index].active_symbols() {
            self.goto_symbol(index, sym);
        }
    }

    fn close_set(
        &self,
        set: &HashSet<Item<'g, T, NT>>,
        already_closed: &mut HashSet<Symbol<T, NT>>,
    ) -> HashSet<Item<'g, T, NT>> {
        // This function only shallowly closes an item. the resulting set may return unclosed items.

        let mut result = HashSet::new();
        // If an item in the set is active we should close on the active symbol,
        // if not it should be ingored.
        for item in set {
            if let Some(sym) = item.active_symbol() {
                // 'already_closed' keeps a record of all previously closed symbols,
                // if the active symbol of 'item' is already in this set it can be assumed that the
                // resulting closure is already present somewhere. Without this, recursion problems would occure.
                if !already_closed.contains(&sym) {
                    result.extend(self.close_item(item));
                    already_closed.insert(sym);
                }
            }
        }

        result
    }

    fn goto_symbol(&mut self, index: usize, sym: Symbol<T, NT>) {
        let mut new_set = ItemSet::new();

        // Create a new set of items from the current item set,
        // where each item has an active symbol equal to sym.
        for item in self.states[index].items() {
            if let Some(active_sym) = item.active_symbol() {
                if active_sym == sym {
                    new_set.insert_kernel(item.advance().unwrap());
                }
            }
        }

        // Check if item set with the same kernel already exists,
        // if so use that index for the transition in the action table,
        // else insert it into the states and use that index.
        let to_state = self.state_index(&new_set).unwrap_or_else(|| {
            self.states.push(new_set);
            self.states.len()
        });

        match sym {
            Symbol::NonTerminal(non_term) => self.table.insert_goto(index, non_term, to_state),
            Symbol::Terminal(term) => {
                self.table
                    .insert_action(index, term, Action::Shift(to_state))
            }
        }
    }

    fn reduce_item(&self, item: &Item<T, NT>) -> (T, Action<T, NT>) {
        // If the rule to reduce is the start rule we should insert an 'Accept' action,
        // if not we insert a normal reduce action.
        if item.rule() == self.grammar.start_rule() {
            (item.look_ahead(), Action::Accept)
        } else {
            (item.look_ahead(), Action::Reduce(item.rule().clone()))
        }
    }

    fn close_item(&self, item: &Item<'g, T, NT>) -> HashSet<Item<'g, T, NT>> {
        // This function only shallowly closes an item. the resulting set may return unclosed items.
        let mut result = HashSet::new();

        // In order to close an item, it must be active and it's active symbol must be a reference to other rules.
        // If this is not the case the resulting closure consists of an empty set.
        if let Some(Symbol::NonTerminal(lhs)) = item.active_symbol() {
            // Find the first set of the symbol following the active symbol,
            // if the item is not active or there is no symbol following the active symbol,
            // use the the look_ahead of this item.
            let look_aheads = self
                .first_set
                .get(&item.following_active())
                .unwrap_or_else(|| {
                    panic!(
                        "Fatal error, first set does not contain {:?}",
                        item.following_active()
                    )
                });

            for rule in self.grammar.rules_by_lhs(lhs) {
                for look_ahead in look_aheads {
                    let item = Item::new(rule, 0, *look_ahead);
                    result.insert(item);
                }
            }
        }

        result
    }
}
