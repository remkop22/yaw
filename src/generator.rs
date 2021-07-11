use crate::analysis::{Action, Table};
use crate::common::Rule;
use codegen::Scope;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

pub fn generate<T, NT>(file_name: &str, table: &Table<T, NT>) -> std::io::Result<()>
where
    T: Hash + Eq + Copy + Debug,
    NT: Hash + Eq + Copy,
{
    let mut scope = Scope::new();
    let str_enum = Regex::new(r#"".*""#).unwrap();

    scope.import("yaw::runtime", "{Parser, Token}");

    let tt_enum = scope.new_enum("TokenType");

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
        .new_struct("GeneratedParser")
        .field("state_stack", "Vec<usize>")
        .field("tokens", "Vec<Token<TokenType>>")
        .field("parse_stack", "Vec<usize>");

    let parser_impl = scope
        .new_impl("GeneratedParser")
        .impl_trait("Parser<TokenType>");

    parser_impl
        .new_fn("next_token")
        .arg_mut_self()
        .ret("Token<TokenType>")
        .line("Token{ span: (0, 0), kind: TokenType::EOF }");

    let action_fn = parser_impl
        .new_fn("action")
        .arg_mut_self()
        .arg("token", "Token<TokenType>")
        .line("match (self.state(), &token.kind) {");

    for (state, nt, action) in unique_actions {
        let action_str = match action {
            Action::Shift(state) => format!("self.shift({}, token)", state),
            Action::Reduce(rule) => format!(
                "self.reduce({})",
                rules.iter().position(|r| r == &rule).unwrap()
            ),
            Action::Accept => "self.accept()".into(),
            Action::Error => "self.error()".into(),
        };

        action_fn.line(format!(
            "({}, TokenType::{:?}) => {},",
            state, nt, action_str
        ));
    }

    action_fn.line("_ => self.error()");
    action_fn.line("}");

    parser_impl
        .new_fn("state")
        .arg_ref_self()
        .ret("usize")
        .line("*self.state_stack.last().unwrap()");

    parser_impl
        .new_fn("push_state")
        .arg_mut_self()
        .arg("state", "usize")
        .line("self.state_stack.push(state);");

    parser_impl
        .new_fn("pop_state")
        .arg_mut_self()
        .line("self.state_stack.pop();");

    parser_impl
        .new_fn("push_token")
        .arg_mut_self()
        .arg("token", "Token<TokenType>")
        .line("self.tokens.push(token);");

    parser_impl
        .new_fn("push_rule")
        .arg_mut_self()
        .arg("rule", "usize")
        .line("self.parse_stack.push(rule);");

    parser_impl
        .new_fn("goto")
        .arg_mut_self()
        .arg("rule", "usize");

    fs::write(file_name, scope.to_string())
}
