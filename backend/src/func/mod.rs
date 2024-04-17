use std::fmt::Debug;

use crate::math::{MathElement, ExpressionElement};

use self::decl::{IntoRawExpr, MathFunction, Prioritizable};

pub mod decl;
pub mod exp_log;
pub mod op;
pub mod trig;

pub trait AsPhantomFunction {}

pub trait PhantomFunction: Debug + IntoRawExpr {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<Option<ExpressionElement>>) -> MathFunction;
}

pub trait Function: Debug + IntoRawExpr + Prioritizable {
    fn evaluate(&self) -> MathElement;
}
