use crate::func::decl::MathFunction;

use self::{expr::ExpressionBuffer, symbol::MathSymbol};

pub mod expr;
pub mod symbol;

#[derive(Debug)]
pub enum MathElement {
    Symbol(MathSymbol),
    Function(MathFunction),
}

#[derive(Debug)]
pub enum ExpressionElement {
    Symbol(MathSymbol),
    Function(MathFunction),
    Expression(ExpressionBuffer),
}
