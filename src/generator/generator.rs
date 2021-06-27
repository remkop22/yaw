use crate::analysis::Table;
use codegen::Scope;
use std::fs;

pub fn generate<T, NT>(file_name: &str, table: &Table<T, NT>) -> std::io::Result<()> {
	let mut scope = Scope::new();

	scope.import("reel::generator", "{Parser, Token}");

	scope.new_enum("TokenType");

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
		.ret("Token<TokenType>");

	parser_impl
		.new_fn("action")
		.arg_mut_self()
		.arg("token", "Token<TokenType>");

	parser_impl.new_fn("state").arg_ref_self().ret("usize");

	parser_impl
		.new_fn("push_state")
		.arg_mut_self()
		.arg("state", "usize");

	parser_impl.new_fn("pop_state").arg_mut_self();
	parser_impl.new_fn("push_token").arg_mut_self().arg("token", "Token<TokenType>");

	parser_impl.new_fn("push_rule").arg_mut_self().arg("rule", "usize");
	parser_impl.new_fn("goto").arg_mut_self().arg("rule", "usize");

	fs::write(file_name, scope.to_string())
}
