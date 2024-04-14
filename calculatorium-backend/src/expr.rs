use std::fmt::Debug;

use crate::{latex::expr::LaTexExpression, sub_expr, DecimalScalar, IntegerScalar};

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

pub trait Function: Debug {
    fn evaluate(&self) -> LaTexExpression;
}

#[derive(Debug)]
pub struct Fraction {
    num: LaTexExpression,
    den: LaTexExpression,
}

impl FromExprs for Fraction {
    fn convert(mut exprs: Vec<LaTexExpression>) -> Self {
        Self {
            num: sub_expr!(exprs, 0),
            den: sub_expr!(exprs, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RealNumber {
    Integer(IntegerScalar),
    Decimal(DecimalScalar),
}

impl FromRawExpr for RealNumber {
    /// Parse `123` `1.024`
    fn parse_raw(expr: &str) -> Option<Self> {
        if expr.is_empty() {
            return None;
        }

        if let Ok(i) = expr.parse::<IntegerScalar>() {
            Some(Self::Integer(i))
        } else {
            expr.parse::<DecimalScalar>().map(|i| Self::Decimal(i)).ok()
        }
    }
}

#[cfg(test)]
mod test {
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
