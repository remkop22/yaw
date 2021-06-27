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

	scope.new_impl("GeneratedParser")
		.impl_trait("Parser")
		.new_fn("next_token");

	fs::write(file_name, scope.to_string())
}
