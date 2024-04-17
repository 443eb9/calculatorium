use crate::func::{
    decl::{IntoRawExpr, Prioritizable},
    Function, PhantomFunction,
};

use self::{
    expr::ExpressionBuffer,
    symbol::{BracketState, Number},
};

pub mod expr;
pub mod symbol;

pub type LaTexParsingResult<T> = Result<T, LaTexParsingError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTexParsingError {
    pub at: u32,
    pub ty: LaTexParsingErrorType,
}

impl LaTexParsingError {
    #[inline]
    pub fn new(at: u32, ty: LaTexParsingErrorType) -> Self {
        Self { at, ty }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaTexParsingErrorType {
    InvalidNumber(String),
    InvalidBracketStructure,
    InvalidFunctionName(String),
    Unknown,
}

#[derive(Debug)]
pub enum ExpressionElement {
    Number(Number),
    Function(Box<dyn Function>),
}

impl IntoRawExpr for ExpressionElement {
    fn assemble(&self) -> String {
        match self {
            ExpressionElement::Number(n) => n.assemble(),
            ExpressionElement::Function(n) => n.assemble(),
        }
    }
}

impl Prioritizable for ExpressionElement {
    fn priority(&self) -> u32 {
        match self {
            ExpressionElement::Number(_) => 1,
            ExpressionElement::Function(f) => f.priority(),
        }
    }
}

#[derive(Debug)]
pub enum MathElement {
    Number(Number),
    Parentheses(BracketState),
    Function(Box<dyn Function>),
    PhantomFunction(Box<dyn PhantomFunction>),
    Expression(ExpressionBuffer),
}

impl IntoRawExpr for MathElement {
    fn assemble(&self) -> String {
        match self {
            MathElement::Number(n) => n.assemble(),
            MathElement::Parentheses(p) => p.assemble(),
            MathElement::Function(f) => f.assemble(),
            MathElement::PhantomFunction(phf) => phf.assemble(),
            MathElement::Expression(e) => e.assemble(),
        }
    }
}
