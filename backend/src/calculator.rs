use crate::{
    math::{expr::ExpresssionTree, func::Function, FromRawExpr, LaTexParsingError, MathElement},
    DecimalScalar,
};

pub type CalculationResult<T> = Result<T, CalculationError>;

pub enum CalculationError {
    Parsing(LaTexParsingError),
}

#[derive(Debug, Default)]
pub struct Calculator {
    // TODO store custom variable etc.
}

impl Calculator {
    pub fn evaluate(&self) -> MathElement {
        todo!()
    }

    pub fn approximate(&self, expr: &str) -> CalculationResult<DecimalScalar> {
        Ok(ExpresssionTree::parse_raw(expr)
            .map_err(|e| CalculationError::Parsing(e))?
            .approximate())
    }
}
