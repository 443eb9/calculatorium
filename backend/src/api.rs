#![allow(unused)]
//! General apis for other projects to invoke.

use crate::{
    func::decl::FromRawExpr,
    math::{expr::LaTexExpression, LaTexParsingResult},
};

pub fn parse(expr: &str) -> LaTexParsingResult<LaTexExpression> {
    LaTexExpression::parse_raw(expr)
}

pub fn evaluate(expr: &str) -> Option<LaTexExpression> {
    todo!()
}

pub fn evaluate_to_latex(expr: &str) -> Option<String> {
    todo!()
}
