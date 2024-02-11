use std::collections::{HashMap, HashSet};

use crate::{list, matcher::nfa::{Key, List, Node, State, Trigger}, parser::{self, Regexp}, state};

use super::{NFABuilder, NFA};

pub struct ThompsonWayBuilder {}

impl NFABuilder for ThompsonWayBuilder {
    fn exec(&self, root: Regexp) -> NFA {
        self.alt(root.val)
    }
}

impl ThompsonWayBuilder {
    pub fn new() -> ThompsonWayBuilder {
        ThompsonWayBuilder{}
    }

    fn alt(&self, ast: parser::Alt) -> NFA {
        let nfa = if ast.val.is_none() {
            self.symbol('ε')
        } else {
            self.concat(ast.val.unwrap())
        };
        if ast.tail.is_none() {
            return nfa;
        }
        let tail = *ast.tail.unwrap();
        let init_state = state!();
        let accepted_state = state!();
        let other = self.alt(tail);
        NFA::new(
            HashSet::from([
                init_state,
                accepted_state,
            ]).into_iter()
            .chain(nfa.states)
            .chain(other.states)
            .collect(), 
            HashMap::from([
                ((init_state, 'ε'), List::from([
                    Node(nfa.init_state, 1), 
                    Node(other.init_state, 1)
                ].into_iter())),
                ((nfa.accepted_state, 'ε'), list!(
                    Node(accepted_state, 1)
                )),
                ((other.accepted_state, 'ε'), list!(
                    Node(accepted_state, 1)
                )),
            ]).into_iter()
            .chain(nfa.transition_table)
            .chain(other.transition_table)
            .collect(), 
            init_state, 
            accepted_state
        )
    }

    fn concat(&self, ast: parser::Concat) -> NFA {
        if ast.tail.is_none() {
            return self.factor(ast.val);
        }
        let tail = *ast.tail.unwrap();
        let left = self.factor(ast.val);
        let right = self.concat(tail);
        let init_state = left.init_state;
        let accepted_state = right.accepted_state;

        let concated_transiions = right
            .transition_table
            .iter()
            .map(|v| {
                if v.0.0 == right.init_state {
                    ((left.accepted_state, v.0.1), v.1.clone())
                } else {
                    (*(v.0), v.1.clone())
                }
            })
            .collect::<HashMap<(State, char), List<Node>>>();
        let transition_table = left
            .transition_table
            .into_iter()
            .chain(concated_transiions.into_iter())
            .collect();
        NFA::new(
            left
            .states
            .into_iter()
            .chain(
                right.states
                .into_iter()
                .filter(|&v| v != right.init_state)
                .collect::<HashSet<State>>()
            )
            .collect(),
            transition_table, 
            init_state, 
            accepted_state
        )
    }

    fn factor(&self, ast: parser::Factor) -> NFA {
        if ast.q.is_some() {
            self.reps(ast)
        } else {
            self.base(ast.val)
        }
    }

    fn reps(&self, ast: parser::Factor) -> NFA {
        let init_state = state!();
        let accepted_state = state!();
        let child = self.base(ast.val);
        let transition_table = HashMap::from([
            ((init_state, 'ε'), List::from([
                Node(accepted_state, 2), 
                Node(child.init_state, 1)
            ].into_iter())),
            ((child.accepted_state, 'ε'), List::from([
                Node(accepted_state, 2), 
                Node(child.init_state, 1)
            ].into_iter())),
        ]).into_iter()
        .chain(child.transition_table.into_iter())
        .collect::<HashMap<(State, Trigger), List<Node>>>();

        NFA::new(
            HashSet::from([init_state, accepted_state])
                .into_iter()
                .chain(child.states)
                .collect(),
            transition_table,
            init_state,
            accepted_state
        )
    }

    fn base(&self, ast: parser::Base) -> NFA {
        match ast {
            parser::Base::Alt(v) => self.alt(*v),
            parser::Base::Char(c) => self.symbol(c.to_char()),
        }
    }

    fn symbol(&self, c: char) -> NFA {
        let init_state = state!();
        let accepted_state = state!();
        let table = match c {
            '.' => {
                (0x20u32..0x7eu32)
                .filter_map(char::from_u32)
                .map(|v| {
                    ((init_state, v), list!(Node(accepted_state, 1)))
                })
                .collect::<HashMap<Key, List<Node>>>()
            },
            _ => HashMap::from([((init_state, c), list!(Node(accepted_state, 1)))])
        };
        NFA::new(
            HashSet::from([init_state, accepted_state]),
            table,
            init_state, 
            accepted_state
        )
    }

}
