use crate::{
    latex::{
        expr::LaTexExpression,
        symbols::{CURLY_BRACKET_L, CURLY_BRACKET_R},
        Position,
    },
    utils::BracketStack,
    DecimalScalar, IntegerScalar,
};

pub trait FromExpr {
    fn parse(expr: &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Fraction {
    num: LaTexExpression,
    den: LaTexExpression,
}

impl FromExpr for Fraction {
    /// Parse `{a}/{b}`
    fn parse(expr: &str) -> Option<Self> {
        let mut num_expr = String::default();
        let mut den_expr = String::default();
        let mut stack = BracketStack::default();

        for c in expr.chars() {
            match &c {
                &CURLY_BRACKET_L => stack.push(Position::Left),
                &CURLY_BRACKET_R => stack.push(Position::Right),
                _ => match stack.depth() {
                    1 => num_expr.push(c),
                    2 => den_expr.push(c),
                    _ => return None,
                },
            }
        }

        let frac = (
            LaTexExpression::parse(&num_expr),
            LaTexExpression::parse(&den_expr),
        );

        if let (Some(num), Some(den)) = frac {
            Some(Self { num, den })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scalar {
    Integer(IntegerScalar),
    Decimal(DecimalScalar),
}

impl FromExpr for Scalar {
    /// Parse `123` `1.024`
    fn parse(expr: &str) -> Option<Self> {
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
        assert_eq!(Scalar::parse("123"), Some(Scalar::Integer(123)));
        assert_eq!(Scalar::parse("1.024"), Some(Scalar::Decimal(1.024)));
        assert_eq!(Scalar::parse("abc"), None);
    }
}
