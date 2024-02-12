use crate::matcher::nfa::{Node, State};

use super::super::{nfa::NFA, Matcher};

pub struct BackTracer{
    nfa: NFA,
}

impl BackTracer {
    pub fn new(nfa: NFA) -> BackTracer {
        BackTracer {
            nfa,
        }
    }

    fn match_dfs(&self, input: &str, cur: State, idx: usize) -> Option<usize> {
        if self.is_accepted(cur) {
            return Some(idx);
        }

        let mut candidates = self.get_next_states(cur, None);
        if let Some(trigger) = input.chars().nth(idx) {
            candidates = candidates.into_iter()
            .chain(self.get_next_states(cur, Some(trigger)).into_iter())
            .collect::<Vec<(bool, Node)>>()
        };
        candidates.sort();

        for i in candidates.into_iter() {
            if let Some(v) = self.match_dfs(input,
                i.1.state(),
                idx + if i.0 {1} else {0}) {
                return Some(v);
            }
        };

        return None;
    }

    fn is_accepted(&self, state: State) -> bool {
        state == self.nfa.get_accepted_state()
    }

    fn get_next_states(&self, cur: State, trigger: Option<char>) -> Vec<(bool, Node)> {
        let (is_consumed, trigger) = match trigger {
            Some(v) => (true, v),
            None => (false, 'ε')
        };
        match self.nfa.get_transition_table().get(&(cur, trigger)) {
            Some(v) => {
                v.iter()
                .map(|v| (is_consumed, v))
                .collect()
            },
            None => Vec::new()
        }
    }
}

impl Matcher for BackTracer {
    fn exec(&self, input: &str) -> Vec<String> {
        let mut begin = 0usize;
        let mut ret = vec![];
        while begin <= input.len() {
            let end = self.match_dfs(input, self.nfa.get_init_state(), begin);
            match end {
                Some(end) => {
                    ret.push(input[begin..end].to_string());
                    begin += if begin < end {
                        end - begin
                    } else {
                        1
                    };
                },
                None => begin += 1,
            };
        }
        ret
    }
}

#[cfg(test)]
mod test;