use std::fmt::Debug;

use super::PhantomFunction;

use crate::{expr::LaTexExpression, latex::LaTexElement};

use calculatorium_macros::{FromExprs, PhantomFunction};

macro_rules! define_get_phfuncs {
    ($($func_name: expr, $func: ty),*) => {
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

define_function!(Fraction, num, den);
define_get_phfuncs!("frac", PhantomFraction);

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
