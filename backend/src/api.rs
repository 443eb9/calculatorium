#![allow(unused)]
//! General apis for other projects to invoke.

use crate::{expr::LaTexExpression, func::decl::FromRawExpr};

pub fn parse(expr: &str) -> Option<LaTexExpression> {
    LaTexExpression::parse_raw(expr)
}

pub fn evaluate(expr: &str) -> Option<LaTexExpression> {
    todo!()
}

pub fn evaluate_to_latex(expr: &str) -> Option<String> {
    todo!()
}
