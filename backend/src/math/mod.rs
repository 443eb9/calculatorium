use crate::func::decl::MathFunction;

use self::{expr::ExpressionBuffer, symbol::MathSymbol};

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
pub enum MathElement {
    Symbol(MathSymbol),
    Function(MathFunction),
}

#[derive(Debug)]
pub enum ExpressionElement {
    Symbol(MathSymbol),
    Function(MathFunction),
    Expression(ExpressionBuffer),
}
