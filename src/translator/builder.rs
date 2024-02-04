use crate::parser::Regexp;

use super::NFA;

pub trait NFABuilder {
    fn exec(ast: Regexp) -> NFA;
}

pub struct ThompsonWayBuilder {}

impl NFABuilder for ThompsonWayBuilder {
    fn exec(ast: Regexp) -> NFA {
        crate::state!();
        todo!();
    }
}

impl ThompsonWayBuilder {
    pub fn new() -> ThompsonWayBuilder {
        ThompsonWayBuilder{}
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};
    use crate::{state, lexer::{self}, parser::{LL0Parser, Parser}, translator::NFA};

    use super::{NFABuilder, ThompsonWayBuilder};

    #[test]
    fn empty_string() {
        let ast = LL0Parser::new().parse(&lexer::get_tokens("")).unwrap();
        let states = HashSet::from([state!(), state!()]);
        let transition_table = HashMap::from([((state!(1), 'ε'), state!(2))]);
        let expected = NFA::new(
            states,
            transition_table,
            state!(1), 
            state!(2),
        );
        let actual = ThompsonWayBuilder::exec(ast);
        assert_eq!(expected, actual);
    }

    #[test]
    fn only_one_char() {
        let ast = LL0Parser::new().parse(&lexer::get_tokens("a")).unwrap();
        let states = HashSet::from([state!(), state!()]);
        let transition_table = HashMap::from([((state!(1), 'a'), state!(2))]);
        let expected = NFA::new(
            states,
            transition_table,
            state!(1), 
            state!(2),
        );
        let actual = ThompsonWayBuilder::exec(ast);
        assert_eq!(expected, actual);
    }
}