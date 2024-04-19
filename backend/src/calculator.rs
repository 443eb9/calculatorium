use std::collections::HashMap;

use crate::{
    math::{
        expr::ExpresssionTree, func::Function, symbol::Number, FromRawExpr, LaTexParsingError,
        MathElement,
    },
    DecimalScalar,
};

pub type CalculationResult<T> = Result<T, CalculationError>;

pub enum CalculationError {
    Parsing(LaTexParsingError),
}

#[derive(Debug, Default)]
pub struct Calculator {
    variables: HashMap<String, Number>,
}

impl Calculator {
    pub fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    pub fn approximate(&self, expr: &str) -> CalculationResult<DecimalScalar> {
        Ok(ExpresssionTree::parse_raw(expr)
            .map_err(|e| CalculationError::Parsing(e))?
            .approximate())
    }

    #[inline]
    pub fn set_variable(&mut self, var: String, val: Number) {
        self.variables.insert(var.into(), val);
    }

    #[inline]
    pub fn get_variable(&self, var: &str) -> Option<Number> {
        self.variables.get(var).cloned()
    }

    #[inline]
    pub fn get_variable_mut(&mut self, var: &str) -> Option<&mut Number> {
        self.variables.get_mut(var)
    }

    #[inline]
    pub fn variables(&self) -> &HashMap<String, Number> {
        &self.variables
    }

    #[inline]
    pub fn variables_mut(&mut self) -> &mut HashMap<String, Number> {
        &mut self.variables
    }
}
