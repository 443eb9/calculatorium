use std::fmt::Debug;

use crate::math::{expr::LaTexExpression, MathElement};

use self::decl::MathFunction;

pub mod decl;

pub trait AsPhantomFunction {}

pub trait PhantomFunction: Debug {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<Option<MathElement>>) -> MathFunction;
}

pub trait Function: Debug {
    fn evaluate(&self) -> LaTexExpression;
}
