use crate::{
    expr::{FromRawExpr, RealNumber},
    sub_expr, take_elem_in_expr,
    utils::BracketStack,
};

use super::{
    symbols::{
        get_phantom_function, LaTexElement, Operator, CURLY_BRACKET_L, CURLY_BRACKET_R, FUNC_BEGIN,
        PARENTHESES_L, PARENTHESES_R,
    },
    BracketState,
};

#[derive(Debug, Default)]
pub struct LaTexExpression {
    raw: Vec<LaTexElement>,
}

impl FromRawExpr for LaTexExpression {
    /// Full-featured LaTex parser.
    fn parse_raw(expr: &str) -> Option<Self> {
        if expr.is_empty() {
            return None;
        }

        let mut symbols = Vec::new();
        let mut digit_buffer = String::new();
        let mut sub_expr_start = -1;
        let mut func_start = -1;
        let mut func_num_params = -1;
        let mut func_param_buffer = Vec::new();
        let mut curly_brackets = BracketStack::default();
        let mut parentheses = BracketStack::default();

        for (i, c) in expr.chars().enumerate() {
            match &c {
                &CURLY_BRACKET_L => curly_brackets.push(BracketState::Open),
                &CURLY_BRACKET_R => curly_brackets.push(BracketState::Close),
                &PARENTHESES_L => parentheses.push(BracketState::Open),
                &PARENTHESES_R => parentheses.push(BracketState::Close),
                _ => {}
            };

            if !curly_brackets.is_valid() || !parentheses.is_valid() {
                return None;
            }

            // Functions
            if func_start != -1 {
                if c == CURLY_BRACKET_L {
                    if let Some(ph_func) = get_phantom_function(&expr[func_start as usize..i]) {
                        symbols.push(LaTexElement::PhantomFunction(ph_func));
                    } else {
                        return None;
                    }
                    func_start = -1;
                } else {
                    continue;
                }
            }

            if c == FUNC_BEGIN {
                func_start = i as i32 + 1;
                continue;
            }

            // Sub Expressions
            if sub_expr_start != -1 {
                if c == CURLY_BRACKET_R {
                    if let Some(sub_expr) =
                        LaTexExpression::parse_raw(&expr[sub_expr_start as usize..i])
                    {
                        if func_num_params != -1 {
                            func_param_buffer.push(sub_expr);
                        } else {
                            symbols.push(LaTexElement::Expression(sub_expr));
                        }
                    } else {
                        return None;
                    }
                    sub_expr_start = -1;
                }
                continue;
            }

            if c == CURLY_BRACKET_L {
                sub_expr_start = i as i32 + 1;
                continue;
            }

            if c.is_digit(10) || c == '.' {
                // Digits
                digit_buffer.push(c);
            } else if c.is_ascii_lowercase() {
                // Custom Variables
                todo!("parse custom variables");
            } else if let Some(scalar) = RealNumber::parse_raw(&digit_buffer) {
                // Real Numbers
                symbols.push(LaTexElement::RealNumber(scalar));
                digit_buffer.clear();
            }

            if let Some(op) = Operator::parse_raw(&c.to_string()) {
                // Operators
                symbols.push(LaTexElement::Operator(op));
            } else {
                match c {
                    CURLY_BRACKET_L => sub_expr_start = i as i32 + 1,
                    // Parentheses
                    PARENTHESES_L | PARENTHESES_R => symbols.push(LaTexElement::Parentheses),
                    _ => {}
                }
            }
        }

        if let Some(scalar) = RealNumber::parse_raw(&digit_buffer) {
            symbols.push(LaTexElement::RealNumber(scalar));
        }

        Some(Self { raw: symbols })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        dbg!(LaTexExpression::parse_raw("15*16+2-9^2/3^{19+2.3}+(5.3+1)"));
    }

    #[test]
    fn test_expr_parser_func() {
        dbg!(LaTexExpression::parse_raw("\\frac{2}{3}"));
    }
}
