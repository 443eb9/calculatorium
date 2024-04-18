use crate::{
    math::func::{
        decl::{IntoRawExpr, Prioritizable},
        Function, Operator, PhantomFunction, PhantomOperator,
    },
    DecimalScalar,
};

use self::{
    expr::ExpressionBuffer,
    symbol::{BracketState, Number},
};

pub mod expr;
pub mod func;
pub mod symbol;

pub type LaTexParsingResult<T> = Result<T, LaTexParsingError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLocation {
    Raw(u32),
    Tokenized(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTexParsingError {
    pub at: ErrorLocation,
    pub ty: ParsingErrorType,
}

impl LaTexParsingError {
    #[inline]
    pub fn new(at: ErrorLocation, ty: ParsingErrorType) -> Self {
        Self { at, ty }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingErrorType {
    EmptyInput,
    InvalidNumber(String),
    InvalidBracketStructure,
    InvalidFunctionName(String),
    InvalidFunctionInvocation(String),
    Unknown,
}

#[derive(Debug)]
pub enum ExpressionElement {
    Number(Number),
    Operator(Box<dyn Operator>),
    Function(Box<dyn Function>),
}

impl IntoRawExpr for ExpressionElement {
    fn assemble(&self) -> String {
        match self {
            ExpressionElement::Number(n) => n.assemble(),
            ExpressionElement::Operator(o) => o.assemble(),
            ExpressionElement::Function(n) => n.assemble(),
        }
    }
}

impl Prioritizable for ExpressionElement {
    fn priority(&self) -> u32 {
        match self {
            ExpressionElement::Number(_) => 1,
            ExpressionElement::Operator(o) => o.priority(),
            ExpressionElement::Function(_) => 10,
        }
    }
}

impl Function for ExpressionElement {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    fn approximate(&self) -> DecimalScalar {
        match self {
            ExpressionElement::Number(n) => n.approximate(),
            ExpressionElement::Operator(op) => op.approximate(),
            ExpressionElement::Function(f) => f.approximate(),
        }
    }
}

#[derive(Debug)]
pub enum MathElement {
    Number(Number),
    Parentheses(BracketState),
    Function(Box<dyn Function>),
    Operator(Box<dyn Operator>),
    PhantomFunction(Box<dyn PhantomFunction>),
    PhantomOperator(Box<dyn PhantomOperator>),
    Expression(ExpressionBuffer),
}
