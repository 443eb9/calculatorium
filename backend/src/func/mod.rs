use std::fmt::Debug;

use crate::math::{ExpressionElement, MathElement};

use self::decl::{IntoRawExpr, Prioritizable};

pub mod decl;
pub mod exp_log;
pub mod op;
pub mod trig;

pub trait AsPhantomFunction {}

pub trait PhantomFunction: Debug + Prioritizable {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<Option<ExpressionElement>>) -> Box<dyn Function>;
}

pub trait Function: Debug + IntoRawExpr {
    fn evaluate(&self) -> MathElement;
}

pub trait PhantomOperator: Debug + Prioritizable {
    fn solidify(&self, params: Vec<Option<ExpressionElement>>) -> Box<dyn Operator>;
}

pub trait Operator: Debug + IntoRawExpr + Prioritizable {
    fn evaluate(&self) -> MathElement;
}
