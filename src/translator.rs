#[macro_use]
pub mod builder;

use std::collections::{HashMap, HashSet};

static mut STATE_ID: u32 = 0;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct State{
    id: u32
}


impl State {
    pub fn new(id: u32) -> State {
        State{id}
    }
}

#[macro_export]
macro_rules! state {
    ($id:expr) => {
        $crate::translator::State::new($id)
    };
    () => {
        {
            unsafe { $crate::translator::STATE_ID += 1 };
            $crate::translator::State::new(unsafe {$crate::translator::STATE_ID})
        }
    };
}

pub type Trigger = char;

#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    states: HashSet::<State>,
    transision_table: HashMap<(State, Trigger), State>,
    init_state: State,
    accepted_state: State,
}

impl NFA {
    pub fn new(
    states: HashSet::<State>,
    transision_table: HashMap<(State, Trigger), State>,
    init_state: State,
    accepted_state: State,) -> NFA {
        NFA { states, transision_table, init_state, accepted_state }
    }

    pub fn transit(&self, q: State, trigger: Trigger) -> Result<State, &'static str> {
        let next = self.transision_table.get(&(q, trigger));
        if let Some(state) = next {
            Ok(*state)
        } else {
            Err("マッチエラー")
        }
    }

}