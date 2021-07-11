use crate::analysis::{Action, Table};
use crate::common::Rule;
use codegen::Scope;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{format, Debug};
use std::fs;
use std::hash::Hash;
use std::ops::Deref;

pub fn generate<T, NT>(file_name: &str, table: &Table<T, NT>) -> std::io::Result<()>
where
    T: Hash + Eq + Copy + Debug,
    NT: Hash + Eq + Copy,
{
    let mut scope = Scope::new();
    let str_enum = Regex::new(r#"".*""#).unwrap();

    let token_type = "TokenType";
    let tokenizer = format!("std::vec::IntoIter<Token<{}>>", token_type);
    let parser_name = "Parser";
    let result = "Result<(), ()>";
    let parse_state = format!("ParseState<{}, {}>", tokenizer, token_type);

    scope.import("yaw::runtime", "{Parse, ParseState, Token}");

    let tt_enum = scope.new_enum(token_type).vis("pub").derive("Copy, Clone");

    let mut unique_terminals = HashSet::new();
    let mut unique_actions = Vec::new();
    let mut unique_rules = HashSet::new();
    for (state, action) in table.actions() {
        for (nt, action) in action {
            let enum_var = format!("{:?}", *nt);
            unique_terminals.insert(String::from(
                str_enum.replace(enum_var.as_str(), "&'static str"),
            ));
            unique_actions.push((state, nt, action));
            if let Action::Reduce(rule) = action {
                unique_rules.insert(rule);
            }
        }
    }

    unique_actions.sort_by_key(|a| *a.0);
    let rules: Vec<&Rule<T, NT>> = unique_rules.into_iter().collect();

    for term in unique_terminals {
        tt_enum.new_variant(term.as_str());
    }

    scope
        .new_struct(parser_name)
        .vis("pub")
        .tuple_field(format!("pub {}", &parse_state));

    let parser_impl = scope
        .new_impl(parser_name)
        .impl_trait(format!("Parse<{}, {}>", tokenizer, token_type));

    parser_impl
        .new_fn("parse_state_mut")
        .arg_mut_self()
        .ret(format!("&mut {}", &parse_state))
        .line("&mut self.0");

    parser_impl
        .new_fn("parse_state")
        .arg_ref_self()
        .ret(format!("&{}", &parse_state))
        .line("&self.0");

    let action_fn = parser_impl
        .new_fn("action")
        .arg_mut_self()
        .ret(format!("Option<{}>", result))
        .arg("token", format!("Token<{}>", token_type))
        .line("match (self.state(), token.kind) {");

    for (state, nt, action) in unique_actions {
        let action_str = match action {
            Action::Shift(state) => format!("self.shift({}, token)", state),
            Action::Reduce(rule) => format!(
                "self.reduce({})",
                rules.iter().position(|r| r == &rule).unwrap()
            ),
            Action::Accept => "return Some(self.accept())".into(),
            Action::Error => "return Some(self.error())".into(),
        };

        action_fn.line(format!(
            "({}, TokenType::{:?}) => {},",
            state, nt, action_str
        ));
    }

    action_fn.line("_ => return Some(self.error())");
    action_fn.line("}");
    action_fn.line("None");

    parser_impl
        .new_fn("goto")
        .arg_mut_self()
        .arg("rule", "usize");

    fs::write(file_name, scope.to_string())
}
