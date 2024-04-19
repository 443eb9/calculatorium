use std::{collections::HashMap, fmt::Display};

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
    fn parse_raw(expr: &str, vars: Option<&HashMap<String, Number>>) -> LaTexParsingResult<Self>
    where
        Self: Sized;

    fn parse_raw_with_base_index(
        expr: &str,
        vars: Option<&HashMap<String, Number>>,
        base: usize,
    ) -> LaTexParsingResult<Self>
    where
        Self: Sized,
    {
        Self::parse_raw(expr, vars).map_err(|mut e| {
            e.at.start += base;
            e
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTexParsingError {
    pub at: MathElementMeta,
    pub ty: LaTexParsingErrorType,
}

impl LaTexParsingError {
    #[inline]
    pub fn new(at: MathElementMeta, ty: LaTexParsingErrorType) -> Self {
        Self { at, ty }
    }

    #[inline]
    pub fn expand(&self, expr: &str) -> String {
        if expr.is_empty() {
            return format!("{}", self.ty);
        }

        format!(
            "{} is a(an) {}",
            &expr[self.at.start..self.at.start + self.at.len],
            self.ty
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaTexParsingErrorType {
    EmptyInput,
    InvalidNumber,
    InvalidConstant,
    InvalidBracketStructure,
    UnknownFunctionName,
    InvalidFunctionCall,
    UnknownCharacter,
    UnknownVariable,
    Unknown,
}

impl Display for LaTexParsingErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathElementMeta {
    pub start: usize,
    pub len: usize,
}

impl MathElementMeta {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn at(at: usize) -> Self {
        Self { start: at, len: 1 }
    }
}

impl From<std::ops::Range<usize>> for MathElementMeta {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            len: value.len(),
        }
    }
}
