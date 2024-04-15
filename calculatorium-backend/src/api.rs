use crate::{expr::LaTexExpression, func::decl::FromRawExpr};

pub fn parse(expr: &str) -> Option<LaTexExpression> {
    LaTexExpression::parse_raw(expr)
}
