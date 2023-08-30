use anyhow::Result;

mod env;
mod eval;
mod lexer;
mod lisp_expr;
mod parser;
mod token;
mod repl;

fn main() -> Result<()> {
    repl::run()
}