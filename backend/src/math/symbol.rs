use std::{collections::HashMap, fmt::Display};

use crate::{
    latex::{E, PI},
    math::{
        func::Function, FromRawExpr, IntoRawExpr, LaTexParsingError, LaTexParsingErrorType,
        LaTexParsingResult, MathElement, MathElementMeta,
    },
    DecimalScalar, IntegerScalar,
};

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

impl Function for Constant {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    fn approximate(&self) -> DecimalScalar {
        match *self {
            // TODO automatically adapt to the target accuracy
            Constant::Pi => std::f64::consts::PI,
            Constant::E => std::f64::consts::E,
        }
    }
}

impl IntoRawExpr for Constant {
    fn assemble(&self) -> String {
        todo!()
    }
}

impl FromRawExpr for Constant {
    fn parse_raw(expr: &str, _: Option<&HashMap<String, Number>>) -> LaTexParsingResult<Self> {
        match expr {
            PI => Ok(Self::Pi),
            E => Ok(Self::E),
            _ => Err(LaTexParsingError::new(
                (0..expr.len()).into(),
                LaTexParsingErrorType::InvalidConstant,
            )),
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

impl Function for Number {
    fn evaluate(&self) -> super::MathElement {
        todo!()
    }

    fn approximate(&self) -> DecimalScalar {
        match *self {
            Number::Integer(i) => i as DecimalScalar,
            Number::Decimal(d) => d,
            Number::Constant(c) => c.approximate(),
        }
    }
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

impl FromRawExpr for Number {
    fn parse_raw(expr: &str, _: Option<&HashMap<String, Number>>) -> LaTexParsingResult<Self> {
        if expr.is_empty() {
            return Err(LaTexParsingError::new(
                MathElementMeta::at(0),
                LaTexParsingErrorType::InvalidNumber,
            ));
        }

        if let Ok(i) = expr.parse::<IntegerScalar>() {
            Ok(Self::Integer(i))
        } else {
            expr.parse::<DecimalScalar>()
                .map(|i| Self::Decimal(i))
                .map_err(|_| {
                    LaTexParsingError::new(
                        (0..expr.len()).into(),
                        LaTexParsingErrorType::InvalidNumber,
                    )
                })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scalar_parser() {
        assert_eq!(
            Number::parse_raw("123", None).unwrap(),
            Number::Integer(123)
        );
        assert_eq!(
            Number::parse_raw("1.024", None).unwrap(),
            Number::Decimal(1.024)
        );
        assert_eq!(
            Number::parse_raw("abc", None).unwrap_err(),
            LaTexParsingError::new((0..3).into(), LaTexParsingErrorType::InvalidNumber)
        );
    }
}
