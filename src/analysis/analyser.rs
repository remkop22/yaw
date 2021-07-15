use crate::analysis::{Action, Table};
use crate::common::{FirstSet, Grammar, Item, Symbol, Terminal};

use std::collections::{HashMap, HashSet};

use std::hash::Hash;

trait ItemSet<'r, T, NT> {
    fn active_symbols(&mut self) -> HashSet<Symbol<T, NT>>;
}

impl<'r, A, T, NT> ItemSet<'r, T, NT> for A
where
    A: Iterator<Item = &'r Item<'r, T, NT>>,
    T: 'static + Copy + Eq + Hash,
    NT: 'static + Copy + Eq + Hash,
{
    fn active_symbols(&mut self) -> HashSet<Symbol<T, NT>> {
        self.filter_map(|i| i.active_symbol()).collect()
    }
}

pub struct Analyser<'g, T, NT> {
    //table: Table<T, NT>,
    //states: Vec<ItemSet<'g, T, NT>>,
    grammar: &'g Grammar<T, NT>,
    first_set: FirstSet<T, NT>,
}

impl<'g, T, NT> Analyser<'g, T, NT>
where
    T: 'static + Terminal,
    NT: 'static + Eq + Hash + Copy + std::fmt::Debug,
{
    pub fn new(grammar: &'g Grammar<T, NT>) -> Self {
        Self {
            grammar,
            first_set: grammar.first_set(),
            //table: Table::new(),
            //states: vec![start_set],
        }

        //analyser.generate_states();
    }

    pub fn states(&self) -> Vec<HashSet<Item<'g, T, NT>>> {
        let start_item = Item::new(self.grammar.start_rule(), 0, T::eof());
        let mut kernels = vec![vec![start_item].into_iter().collect::<HashSet<_>>()];
        let mut closures = vec![];
        let mut itemsets = vec![];

        while kernels.len() > closures.len() {
            for i in 0..kernels.len() {
                closures.push(self.close(&kernels[i]));

                let state = kernels[i]
                    .union(&closures[i])
                    .cloned()
                    .collect::<HashSet<_>>();

                for sym in state.iter().active_symbols() {
                    let new_kernel = self.goto_symbol(&state, sym);
                    if let Some(i) = kernels.iter().position(|k| k == &kernels[i]) {
                    } else {
                        kernels.push(new_kernel);
                    }
                }

                itemsets.push(state);
            }
        }

        itemsets
    }

    pub fn table(grammar: &'g Grammar<T, NT>) -> Table<T, NT> {
        let analyzer = Self::new(grammar);
        todo!();
    }

    fn close(&self, state: &HashSet<Item<'g, T, NT>>) -> HashSet<Item<'g, T, NT>> {
        let mut already_closed = HashSet::new();

        // Fill the queue with initial closure of the kernel.
        let mut queue = self.close_once(state, &mut already_closed);

        let mut new_queue = HashSet::new();
        loop {
            // Fill the empty new_queue 'buffer' with the closure of the current queue.
            new_queue.extend(self.close_once(&queue, &mut already_closed));

            // If the above operation yields no new closures, it means that the current queue is fully closed,
            // if not it we should add the new items to the queue to be closed in the next loop.
            if new_queue.is_empty() {
                break;
            } else {
                queue.extend(&new_queue);
                new_queue.clear();
            }
        }

        queue
    }

    // fn goto_state(&mut self, index: usize) {
    //     for sym in self.states[index].active_symbols() {
    //         self.goto_symbol(index, sym);
    //     }
    // }

    fn close_once(
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

    fn goto_symbol(
        &self,
        state: &HashSet<Item<'g, T, NT>>,
        sym: Symbol<T, NT>,
    ) -> HashSet<Item<'g, T, NT>> {
        let mut new_set = HashSet::new();

        // Create a new set of items from the current item set,
        // where each item has an active symbol equal to sym.
        for item in state {
            if let Some(active_sym) = item.active_symbol() {
                if active_sym == sym {
                    new_set.insert(item.advance().unwrap());
                }
            }
        }

        new_set

        // match sym {
        //     Symbol::NonTerminal(non_term) => self.table.insert_goto(index, non_term, to_state),
        //     Symbol::Terminal(term) => {
        //         self.table
        //             .insert_action(index, term, Action::Shift(to_state))
        //     }
        // }
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
