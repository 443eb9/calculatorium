use std::fmt::Debug;

use super::PhantomFunction;

use crate::{
    expr::LaTexExpression,
    latex::{symbol::*, LaTexElement},
};

use calculatorium_macros::{FromExprs, PhantomFunction};

macro_rules! define_get_phfuncs {
    ($($func_name: pat, $func: ty, $num_params: literal),*) => {
        pub fn get_phantom_function(name: &str) -> Option<Box<dyn PhantomFunction>> {
            match name {
                $($func_name => Some(Box::new(<$func>::default())),)*
                _ => None
            }
        }
    };
}

pub trait FromRawExpr {
    fn parse_raw(expr: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait FromExprs {
    fn convert(exprs: Vec<LaTexExpression>) -> Self
    where
        Self: Sized;
}

macro_rules! define_function {
    ($fn_ty: ident, $($field: ident),*) => {
        #[derive(Debug, FromExprs, PhantomFunction)]
        pub struct $fn_ty {
            $($field: LaTexExpression,)*
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
define_get_phfuncs!(
    ADD, PhantomAdd, 2,
    SUBTRACT, PhantomSubtract, 2,
    MULTIPLY, PhantomMultiply, 2,
    DIVIDE, PhantomDivide, 2,
    SUPER_SCRIPT, PhantomPower, 2,

    FRAC, PhantomFraction, 2,
    ROOT, PhantomRoot, 2,
    
    LOG, PhantomLog, 2,
    LG, PhantomLg, 1,
    LN, PhantomLn, 1,

    SIN, PhantomSin, 1,
    COS, PhantomCos, 1,
    TAN, PhantomTan, 1
);

#[cfg(test)]
mod test {
    use crate::symbol::RealNumber;

    use super::*;

    #[test]
    fn test_scalar_parser() {
        assert_eq!(RealNumber::parse_raw("123"), Some(RealNumber::Integer(123)));
        assert_eq!(
            RealNumber::parse_raw("1.024"),
            Some(RealNumber::Decimal(1.024))
        );
        assert_eq!(RealNumber::parse_raw("abc"), None);
    }
}
