use std::fmt::Debug;

use crate::{
    math::{ExpressionElement, IntoRawExpr, MathElement, Prioritizable},
    DecimalScalar,
};

use self::decl::MathFunction;

pub mod decl;
pub mod exp_log;
pub mod integ;
pub mod op;
pub mod sp;
pub mod trig;

pub trait AsPhantomFunction {}

pub trait PhantomFunction: Debug + Prioritizable {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<Option<ExpressionElement>>) -> MathFunction;
}

pub trait Function: Debug + IntoRawExpr {
    fn evaluate(&self) -> MathElement;
    fn approximate(&self) -> DecimalScalar;
}

pub trait PhantomOperator: Debug + Prioritizable {
    fn solidify(&self, params: Vec<Option<ExpressionElement>>) -> MathFunction;
}

pub trait Operator: Debug + IntoRawExpr + Prioritizable + Function {}
