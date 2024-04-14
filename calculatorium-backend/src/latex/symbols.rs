use std::fmt::Debug;

use crate::expr::{Fraction, FromExprs, FromRawExpr, RealNumber};

use super::expr::LaTexExpression;

pub const FUNC_BEGIN: char = '\\';

pub const FRACTION: &str = "frac";
pub const SQRT: &str = "sqrt";

pub const PARENTHESES_L: char = '(';
pub const PARENTHESES_R: char = ')';
pub const CURLY_BRACKET_L: char = '{';
pub const CURLY_BRACKET_R: char = '}';

pub trait PhantomFunction: Debug {
    fn num_params(&self) -> u32;
    fn solidify(&self, params: Vec<LaTexExpression>) -> LaTexElement;
}

macro_rules! define_phfuncs {
    ($($func: ident, $solid: ty, $elem: ident, $num_params: expr),*) => {
        $(
            #[derive(Debug, Default)]
            pub struct $func;

            impl PhantomFunction for $func {
                #[inline]
                fn num_params(&self) -> u32 {
                    $num_params
                }

                #[inline]
                fn solidify(&self, params: Vec<LaTexExpression>) -> LaTexElement {
                    LaTexElement::$elem(<$solid>::convert(params))
                }
            }
        )*
    };
}

macro_rules! get_phfuncs {
    ($name: expr, $($func_name: expr, $func: ty),*) => {
        match $name {
            $(
                $func_name => Some(Box::new(<$func>::default())),
            )*
            _ => None
        }
    };
}

define_phfuncs!(PhantomFraction, Fraction, Fraction, 2);

pub fn get_phantom_function(name: &str) -> Option<Box<dyn PhantomFunction>> {
    get_phfuncs!(name, "frac", PhantomFraction)
}

#[derive(Debug, Default)]
pub enum LaTexElement {
    RealNumber(RealNumber),
    Operator(Operator),
    Root(Root),
    Fraction(Fraction),
    Logarithm(Logarithm),
    Trigonometric(Trigonometric),
    Parentheses,
    Expression(LaTexExpression),
    PhantomFunction(Box<dyn PhantomFunction>),
    #[default]
    Dummy,
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
