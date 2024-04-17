use std::fmt::Display;

use crate::{math::func::decl::IntoRawExpr, DecimalScalar, IntegerScalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketState {
    Open,
    Close,
}

impl IntoRawExpr for BracketState {
    fn assemble(&self) -> String {
        match self {
            BracketState::Open => format!("("),
            BracketState::Close => format!(")"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constant {
    Pi,
    E,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Constant::Pi => "\\pi",
            Constant::E => "e",
        })
    }
}

impl Constant {
    pub fn parse_raw(expr: &str) -> Option<Self> {
        match expr {
            "\\pi" => Some(Self::Pi),
            "e" => Some(Self::E),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(IntegerScalar),
    Decimal(DecimalScalar),
    Constant(Constant),
    // Virtual()
}

impl IntoRawExpr for Number {
    fn assemble(&self) -> String {
        match self {
            Number::Integer(i) => format!("{}", i),
            Number::Decimal(d) => format!("{}", d),
            Number::Constant(c) => format!("{}", c),
        }
    }
}

impl Number {
    pub fn parse_raw(expr: &str) -> Option<Self> {
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
    use crate::math::symbol::Number;

    #[test]
    fn test_scalar_parser() {
        assert_eq!(Number::parse_raw("123"), Some(Number::Integer(123)));
        assert_eq!(Number::parse_raw("1.024"), Some(Number::Decimal(1.024)));
        assert_eq!(Number::parse_raw("abc"), None);
    }
}
