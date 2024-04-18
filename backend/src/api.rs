//! General apis for other projects to call.

use crate::{
    math::{expr::ExpresssionTree, func::Function, FromRawExpr, LaTexParsingResult},
    DecimalScalar,
};

pub fn approximate(expr: &str) -> LaTexParsingResult<DecimalScalar> {
    Ok(ExpresssionTree::parse_raw(expr)?.approximate())
}
