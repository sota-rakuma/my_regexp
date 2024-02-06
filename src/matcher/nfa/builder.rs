use crate::parser::Regexp;

use super::NFA;

pub mod thompson;

pub trait NFABuilder {
    fn exec(&self, node: Regexp) -> NFA;
}