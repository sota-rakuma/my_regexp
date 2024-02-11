#[macro_use]
pub mod nfa;
pub mod executor;

pub trait Matcher {
    fn exec(&self, input: & str) -> Vec<String>;
}