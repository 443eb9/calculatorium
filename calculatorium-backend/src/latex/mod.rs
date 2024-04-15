use crate::{
    expr::LaTexExpression,
    func::{
        decl::{Fraction, FromRawExpr},
        PhantomFunction,
    },
    symbol::RealNumber,
};

pub mod expr;
pub mod symbols;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketState {
    Open,
    Close,
}

#[derive(Debug, Default)]
pub enum LaTexElement {
    RealNumber(RealNumber),
    Operator(Operator),
    Root(Root),
    Fraction(Fraction),
    Logarithm(Logarithm),
    Trigonometric(Trigonometric),
    Parentheses(BracketState),
    Expression(LaTexExpression),
    PhantomFunction(Box<dyn PhantomFunction>),
    #[default]
    Dummy,
}

pub const FUNC_BEGIN: char = '\\';

pub const FRACTION: &str = "frac";
pub const SQRT: &str = "sqrt";

pub const PARENTHESES_L: char = '(';
pub const PARENTHESES_R: char = ')';
pub const CURLY_BRACKET_L: char = '{';
pub const CURLY_BRACKET_R: char = '}';

pub const ADD: char = '+';
pub const SUBTRACT: char = '-';
pub const MULTIPLY: char = '*';
pub const DIVIDE: char = '/';
pub const SUPER_SCRIPT: char = '^';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl FromRawExpr for Operator {
    fn parse_raw(expr: &str) -> Option<Self> {
        if expr.len() != 1 {
            return None;
        }

        let expr = expr.chars().next().unwrap();
        match &expr {
            &ADD => Some(Self::Add),
            &SUBTRACT => Some(Self::Subtract),
            &MULTIPLY => Some(Self::Multiply),
            &DIVIDE => Some(Self::Divide),
            &SUPER_SCRIPT => Some(Self::Power),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Root {
    Square,
    Custom(RealNumber),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigonometric {
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Csc,
    Sec,
    Cot,
    Arccsc,
    Arcsec,
    Arccot,

    Sinh,
    Cosh,
    Tanh,
    Arcsinh,
    Arccosh,
    Arctanh,
    Csch,
    Sech,
    Coth,
    Arccsch,
    Arcsech,
    Arccoth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logarithm {
    Log,
    Lg,
    Ln,
}
