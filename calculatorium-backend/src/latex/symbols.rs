use crate::expr::{Fraction, FromExpr, Scalar};

pub const FUNC_BEGIN: char = '\\';

pub const FRACTION: &str = "frac";

pub const PARENTHESES_L: char = '(';
pub const PARENTHESES_R: char = ')';
pub const CURLY_BRACKET_L: char = '{';
pub const CURLY_BRACKET_R: char = '}';

#[derive(Debug, Clone)]
pub enum LaTexSymbol {
    Scalar(Scalar),
    Operator(Operator),
    Root(Root),
    Fracion(Fraction),
    Logarithm(Logarithm),
    Trigonometric(Trigonometric),
    Parentheses,
}

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

impl FromExpr for Operator {
    fn parse(expr: &str) -> Option<Self> {
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
    Custom(Scalar),
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
