use crate::{func::decl::FromRawExpr, DecimalScalar, IntegerScalar};

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
