use crate::{DecimalScalar, IntegerScalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketState {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy)]
pub enum MathSymbol {
    Number(Number),
    Parentheses(BracketState),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(IntegerScalar),
    Decimal(DecimalScalar),
    // Virtual()
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
