use crate::common::{Rule, Symbol, Terminal};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Grammar<T, NT> {
    rules: Vec<Rule<T, NT>>,
    start_rule: usize,
}

impl<'sr, T, NT> Grammar<T, NT>
where
    T: Terminal,
    NT: Eq + Hash + Copy,
{
    pub fn new(rules: Vec<Rule<T, NT>>, start_rule: usize) -> Self {
        Self { rules, start_rule }
    }

    pub fn rules_by_lhs(&self, lhs: NT) -> Vec<&Rule<T, NT>> {
        self.rules.iter().filter(|r| r.lhs() == lhs).collect()
    }

    pub fn rules(&self) -> &Vec<Rule<T, NT>> {
        &self.rules
    }

    pub fn start_rule(&self) -> &Rule<T, NT> {
        &self.rules[self.start_rule]
    }

    fn unique_symbols(&self) -> HashSet<Symbol<T, NT>> {
        self.rules
            .iter()
            .flat_map(|r| {
                let mut syms = r.symbols().clone();
                syms.push(r.lhs_as_sym());
                syms
            })
            .chain(std::iter::once(Symbol::Terminal(T::eof())))
            .collect()
    }

    fn unique_terminals(&self) -> HashSet<T> {
        self.unique_symbols()
            .iter()
            .flat_map(|s| s.terminal())
            .collect()
    }

    fn unique_nonterminals(&self) -> HashSet<NT> {
        self.unique_symbols()
            .iter()
            .flat_map(|s| s.non_terminal())
            .collect()
    }

    pub fn first_set(&self) -> HashMap<Symbol<T, NT>, HashSet<T>> {
        let mut first_set = HashMap::new();
        let terms = self.unique_terminals();
        let nonterms = self.unique_nonterminals();
        let mut empty = HashSet::new();

        for nonterm in nonterms {
            first_set.insert(Symbol::NonTerminal(nonterm), HashSet::new());
        }

        for term in terms {
            first_set.insert(Symbol::Terminal(term), vec![term].into_iter().collect());
        }

        loop {
            let mut updated = false;

            for rule in &self.rules {
                let mut lhs_set = first_set[&rule.lhs_as_sym()].clone();
                let mut all_empty = false;

                for sym in rule.symbols() {
                    updated = first_set[sym].is_subset(&lhs_set);
                    lhs_set.extend(&first_set[sym]);
                    if !empty.contains(sym) {
                        all_empty = true;
                        break;
                    }
                }

                if !all_empty {
                    updated = empty.insert(rule.lhs_as_sym());
                }

                first_set.insert(rule.lhs_as_sym(), lhs_set);
            }

            if !updated {
                break;
            }
        }

        first_set
    }
}
