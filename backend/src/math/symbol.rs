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
