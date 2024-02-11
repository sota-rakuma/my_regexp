pub mod builder;

use std::{collections::{HashMap, HashSet}, fmt::Debug};

use crate::utils::list::List;

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
        $crate::matcher::nfa::State::new($id)
    };
    () => {
        {
            unsafe { $crate::matcher::nfa::STATE_ID += 1 };
            $crate::matcher::nfa::State::new(unsafe {$crate::matcher::nfa::STATE_ID})
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node(State, u8);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl Node {
    pub fn state(&self) -> State {
        self.0
    }

    pub fn priority(&self) -> u8 {
        self.1
    }
}

pub type Trigger = char;
pub type Key = (State, char);

#[derive(PartialEq, Eq)]
pub struct NFA {
    states: HashSet::<State>,
    transition_table: HashMap<Key, List<Node>>,
    init_state: State,
    accepted_state: State,
}

impl NFA {
    pub fn new(
    states: HashSet::<State>,
    transition_table: HashMap<Key, List<Node>>,
    init_state: State,
    accepted_state: State,) -> NFA {
        NFA { states, transition_table, init_state, accepted_state }
    }

    pub fn transit(&self, q: State, trigger: Trigger) -> Result<List<Node>, &'static str> {
        let next = self.transition_table.get(&(q, trigger));
        if let Some(state) = next {
            Ok(state.clone())
        } else {
            Err("マッチエラー")
        }
    }

    pub fn get_init_state(&self) -> State {
        self.init_state
    }

    pub fn get_accepted_state(&self) -> State {
        self.accepted_state
    }

    pub fn get_transition_table(&self) -> &HashMap<Key, List<Node>> {
        &self.transition_table
    }
}

impl Debug for NFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NFA")
        .field("states", &self.states)
        .field("transition_table", &self.transition_table)
        .field("init_state", &self.init_state)
        .field("accepted_state", &self.accepted_state)
        .finish()
    }
}