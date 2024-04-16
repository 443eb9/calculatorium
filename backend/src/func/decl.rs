use std::fmt::Debug;

use super::PhantomFunction;

use crate::{
    latex::*,
    math::{LaTexParsingError, LaTexParsingResult, MathElement},
};

use calculatorium_macros::{AsPhantomFunction, FromExpr};

pub trait FromRawExpr {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self>
    where
        Self: Sized;

    fn parse_raw_with_base_index(expr: &str, base: u32) -> LaTexParsingResult<Self>
    where
        Self: Sized,
    {
        Self::parse_raw(expr).map_err(|e| LaTexParsingError::new(e.at + base, e.ty))
    }
}

pub trait FromExpr {
    fn convert(expr: Vec<Option<MathElement>>) -> Self
    where
        Self: Sized;
}

macro_rules! register_functions {
    ($($fn_name: pat, $fn_ident: ident, $fn_ty: ty, $phfn_ty: ty, $num_params: literal),*) => {
        #[derive(Debug)]
        pub enum MathFunction {
            $($fn_ident(Box<$fn_ty>),)*

            Phantom(Box<dyn PhantomFunction>),
        }

        pub fn get_phantom_function(name: &str) -> Option<Box<dyn PhantomFunction>> {
            match name {
                $($fn_name => Some(Box::new(<$phfn_ty>::default())),)*
                _ => None
            }
        }
    };
}

macro_rules! define_function {
    ($fn_ty: ident, $($field: ident),*) => {
        #[derive(Debug, FromExpr, AsPhantomFunction)]
        pub struct $fn_ty {
            $($field: MathElement,)*
        }
    };
}

define_function!(Add, lhs, rhs);
define_function!(Subtract, lhs, rhs);
define_function!(Multiply, lhs, rhs);
define_function!(Divide, lhs, rhs);
define_function!(Power, base, exp);

define_function!(Fraction, num, den);
define_function!(Root, rad, deg);

define_function!(Log, base, anti);
define_function!(Lg, anti);
define_function!(Ln, anti);

define_function!(Sin, x);
define_function!(Cos, x);
define_function!(Tan, x);

#[rustfmt::skip]
register_functions!(
    ADD, Add, Add, PhantomAdd, 2,
    SUBTRACT, Subtract, Subtract, PhantomSubtract, 2,
    MULTIPLY, Multiply, Multiply, PhantomMultiply, 2,
    DIVIDE, Divide, Divide, PhantomDivide, 2,
    SUPER_SCRIPT, Power, Power, PhantomPower, 2,

    FRAC, Fraction, Fraction, PhantomFraction, 2,
    ROOT, Root, Root, PhantomRoot, 2,
    
    LOG, Log, Log, PhantomLog, 2,
    LG, Lg, Lg, PhantomLg, 1,
    LN, Ln, Ln, PhantomLn, 1,

    SIN, Sin, Sin, PhantomSin, 1,
    COS, Cos, Cos, PhantomCos, 1,
    TAN, Tan, Tan, PhantomTan, 1
);

#[cfg(test)]
mod test {
    use crate::math::symbol::Number;

    #[test]
    fn test_scalar_parser() {
        assert_eq!(Number::parse_raw("123"), Some(Number::Integer(123)));
        assert_eq!(Number::parse_raw("1.024"), Some(Number::Decimal(1.024)));
        assert_eq!(Number::parse_raw("abc"), None);
    }
}
