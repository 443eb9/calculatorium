use crate::{
    math::func::{Function, Operator, PhantomFunction, PhantomOperator},
    DecimalScalar,
};

use self::{
    expr::ExpressionBuffer,
    symbol::{BracketState, Number},
};

pub mod expr;
pub mod func;
pub mod symbol;

pub trait FromExpr {
    fn convert(expr: Vec<Option<ExpressionElement>>) -> Self
    where
        Self: Sized;
}

pub trait FromRawExpr {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self>
    where
        Self: Sized;

    fn parse_raw_with_base_index(expr: &str, base: u32) -> LaTexParsingResult<Self>
    where
        Self: Sized,
    {
        Self::parse_raw(expr).map_err(|e| {
            LaTexParsingError::new(
                match e.at {
                    ErrorLocation::Raw(i) => ErrorLocation::Raw(i + base),
                    ErrorLocation::Tokenized(i) => ErrorLocation::Tokenized(i + base),
                },
                e.ty,
            )
        })
    }
}

pub trait IntoRawExpr {
    fn assemble(&self) -> String;
}

pub trait Prioritizable {
    /// The higher, the earlier it is evaluated.
    /// - Add/Subtract/Binary/Modulo Operations 1
    /// - Multiply/Divide 5
    /// - Functions (Power, Log, Sin etc) 10
    fn priority(&self) -> u32;
}

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
