pub mod thompson;

use crate::{lexer::get_tokens, matcher::nfa::{builder::NFABuilder, NFA}, parser::Parser};

fn create_nfa(
    builder: &impl NFABuilder,
    parser: &mut impl Parser, 
    pattern: &str)
    -> NFA {
    let ast = parser.parse(&get_tokens(pattern)).unwrap();
    builder.exec(ast)
}