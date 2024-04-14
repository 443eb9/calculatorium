use crate::{
    expr::{Fraction, FromExpr, Scalar},
    utils::BracketStack,
    IntegerScalar,
};

use super::symbols::{
    LaTexSymbol, Operator, ADD, CURLY_BRACKET_L, CURLY_BRACKET_R, FRACTION, FUNC_BEGIN,
    PARENTHESES_L, PARENTHESES_R, SUPER_SCRIPT,
};

#[derive(Debug, Clone)]
pub struct LaTexExpression {
    raw: Vec<LaTexSymbol>,
}

impl FromExpr for LaTexExpression {
    /// Full-featured LaTex parser.
    fn parse(expr: &str) -> Option<Self> {
        todo!()
    }
}

impl LaTexExpression {
    pub fn parse_symbol(symbol: &str) -> Option<LaTexSymbol> {
        if symbol.is_empty() {
            return None;
        }

        if symbol.starts_with(FUNC_BEGIN) {
            let expr = &symbol[1..];
            match expr {
                FRACTION => Fraction::parse(expr).map(|f| LaTexSymbol::Fracion(f)),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Parse expressions without functions like `\frac{a}{b}`
    pub fn parse_simple_expr(expr: &str) -> Option<Vec<LaTexSymbol>> {
        let mut symbols = Vec::new();
        let mut digit_buffer = String::new();
        let mut letter_buffer = String::new();
        let mut sub_expr_start = -1;

        for (i, c) in expr.chars().enumerate() {
            if sub_expr_start != -1 {
                if c == CURLY_BRACKET_R {
                    if let Some(sub_expr) =
                        LaTexExpression::parse_simple_expr(&expr[sub_expr_start as usize..i])
                    {
                        symbols.push(LaTexSymbol::Parentheses);
                        symbols.extend(sub_expr);
                        symbols.push(LaTexSymbol::Parentheses);
                        sub_expr_start = -1;
                    } else {
                        return None;
                    }
                }
                continue;
            }

            if c.is_digit(10) || c == '.' {
                digit_buffer.push(c);
            } else if c.is_ascii_lowercase() {
                todo!("parse custom variables");
            } else if let Some(scalar) = Scalar::parse(&digit_buffer) {
                symbols.push(LaTexSymbol::Scalar(scalar));
                digit_buffer.clear();
            }

            if let Some(op) = Operator::parse(&c.to_string()) {
                symbols.push(LaTexSymbol::Operator(op));
            } else {
                match c {
                    CURLY_BRACKET_L => sub_expr_start = i as i32 + 1,
                    PARENTHESES_L | PARENTHESES_R => symbols.push(LaTexSymbol::Parentheses),
                    _ => {}
                }
            }
        }

        if let Some(scalar) = Scalar::parse(&digit_buffer) {
            symbols.push(LaTexSymbol::Scalar(scalar));
        }

        Some(symbols)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_simple_expr() {
        dbg!(LaTexExpression::parse_simple_expr(
            "15*16+2-9^2/3^{19+2.3}+(5.3+1)"
        ));
    }
}
