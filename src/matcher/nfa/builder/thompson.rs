use std::collections::{HashMap, HashSet};

use crate::{matcher::nfa::{State, Trigger}, parser::{self, Regexp}, state};

use super::{NFABuilder, NFA};

pub struct ThompsonWayBuilder {}

// concat: AB(A, B ∈ 正規言語), 初期状態: A, 受理状態: B
//   update:
//     遷移表:
//       Aの受理状態のレコードにBの初期状態の遷移関数を全てくっつけて、Bの初期状態を消す
// altanative: A | B(A, B ∈ 正規言語), 初期状態: new, 受理状態: new
//   update:
//     遷移表:
//       新たな初期状態Q1からεをトリガーとして、AとBの初期状態に向かう遷移関数を遷移表に加える
//       AとBの受理状態からεをトリガーとして、新たな受理状態Qnに向かう遷移関数を遷移表に加える
// kleene closure: A*(A ∈ 正規言語), 初期状態: new, 受理状態: new
//   update:
//      遷移表:
//        新たな初期状態Q1からεをトリガーとして、新たな受理状態Qnに向かう遷移関数を遷移表に加える
//        新たな初期状態Q1からεをトリガーとして、Aの初期状態に向かう遷移関数を遷移表に加える
//        Aの受理状態からεをトリガーとして、新たな受理状態Qnに向かう遷移関数を遷移表に加える
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
                ((init_state, 'ε'), nfa.init_state),
                ((init_state, 'ε'), other.init_state),
                ((nfa.accepted_state, 'ε'), accepted_state),
                ((other.accepted_state, 'ε'), other.init_state),
            ]).into_iter()
            .chain(nfa.transision_table)
            .chain(other.transision_table)
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
            .transision_table
            .iter()
            .map(|v| {
                if v.0.0 == right.init_state {
                    ((left.accepted_state, v.0.1), *v.1)
                } else {
                    (*(v.0), *v.1)
                }
            })
            .collect::<HashMap<(State, char), State>>();
        let transision_table = left
            .transision_table
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
            transision_table, 
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
            ((init_state, 'ε'), accepted_state),
            ((init_state, 'ε'), child.init_state),
            ((child.accepted_state, 'ε'), accepted_state),
        ]).into_iter()
        .chain(child.transision_table)
        .collect::<HashMap<(State, Trigger), State>>();

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
        NFA::new(
            HashSet::from([init_state, accepted_state]),
            HashMap::from([((init_state, c), accepted_state)]), 
            init_state, 
            accepted_state
        )
    }

}