use std::fmt::Debug;

use crate::{expr::LaTexExpression, latex::LaTexElement};

pub mod decl;

pub trait PhantomFunction: Debug {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<LaTexExpression>) -> LaTexElement;
}

pub trait Function: Debug {
    fn evaluate(&self) -> LaTexExpression;
}
