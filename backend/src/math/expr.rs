use crate::{
    func::decl::{get_phantom_function, FromRawExpr, MathFunction},
    latex::*,
    math::symbol::{BracketState, Number},
    utils::BracketStack,
};

use super::{symbol::MathSymbol, ExpressionElement};

#[derive(Debug)]
pub struct ExpressionBuffer {
    pub expr: Vec<ExpressionElement>,
}

impl FromRawExpr for ExpressionBuffer {
    fn parse_raw(expr: &str) -> Option<Self> {
        let mut expr_buffer = Vec::new();

        let mut digit_start = -1;
        let mut func_sub_expr_start = -1;
        let mut func_sub_expr_start_depth = 0;
        let mut user_sub_expr_start = -1;
        let mut user_sub_expr_start_depth = 0;
        let mut func_def_start = -1;

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

            // Function Sub Expressions
            if func_sub_expr_start != -1
                && c == CURLY_BRACKET_R
                && curly_brackets.depth() == func_sub_expr_start_depth
            {
                if let Some(sub_expr) =
                    ExpressionBuffer::parse_raw(&expr[func_sub_expr_start as usize..i])
                {
                    expr_buffer.push(ExpressionElement::Expression(sub_expr));
                    func_sub_expr_start = -1;
                    continue;
                }
            }

            // User Sub Expressions
            if user_sub_expr_start != -1
                && c == PARENTHESES_R
                && parentheses.depth() == user_sub_expr_start_depth
            {
                if let Some(sub_expr) =
                    ExpressionBuffer::parse_raw(&expr[user_sub_expr_start as usize..i])
                {
                    expr_buffer.push(ExpressionElement::Expression(sub_expr));
                    user_sub_expr_start = -1;
                    continue;
                }
            }

            // Functions
            if func_def_start != -1 && c == CURLY_BRACKET_L {
                if let Some(ph_func) = get_phantom_function(&expr[func_def_start as usize..i]) {
                    expr_buffer.push(ExpressionElement::Function(MathFunction::Phantom(ph_func)));
                    func_def_start = -1;
                } else {
                    return None;
                }
            }

            if func_sub_expr_start != -1 || user_sub_expr_start != -1 || func_def_start != -1 {
                continue;
            }

            if c == CURLY_BRACKET_L {
                func_sub_expr_start = i as i32 + 1;
                func_sub_expr_start_depth = curly_brackets.depth() - 1;
                continue;
            }

            if c == PARENTHESES_L {
                user_sub_expr_start = i as i32 + 1;
                user_sub_expr_start_depth = parentheses.depth() - 1;
                continue;
            }

            if c == FUNC_BEGIN {
                func_def_start = i as i32 + 1;
                continue;
            }

            // Digits
            if c.is_digit(10) || c == '.' {
                digit_start = i as i32;
                continue;
            }

            // Custom Variables
            if c.is_ascii_lowercase() {
                todo!("parse custom variables");
                // continue;
            }

            // Real Numbers
            if digit_start != -1 {
                if let Some(scalar) = Number::parse_raw(&expr[digit_start as usize..i]) {
                    expr_buffer.push(ExpressionElement::Symbol(MathSymbol::Number(scalar)));
                    digit_start = -1;
                }
            }

            // Operators
            if matches!(
                c.to_string().as_str(),
                ADD | SUBTRACT | MULTIPLY | DIVIDE | SUPER_SCRIPT
            ) {
                if matches!(c.to_string().as_str(), ADD | SUBTRACT) && expr_buffer.is_empty() {
                    expr_buffer.push(ExpressionElement::Symbol(MathSymbol::Number(
                        Number::Integer(0),
                    )));
                }

                expr_buffer.push(ExpressionElement::Function(MathFunction::Phantom(
                    get_phantom_function(c.to_string().as_str()).unwrap(),
                )));
                continue;
            }

            return None;
        }

        if digit_start != -1 {
            if let Some(scalar) = Number::parse_raw(&expr[digit_start as usize..]) {
                expr_buffer.push(ExpressionElement::Symbol(MathSymbol::Number(scalar)));
            } else {
                return None;
            }
        }

        Some(Self { expr: expr_buffer })
    }
}

#[derive(Debug)]
pub struct LaTexExpression {
    root: MathFunction,
}

impl FromRawExpr for LaTexExpression {
    /// Full-featured LaTex parser.
    fn parse_raw(expr: &str) -> Option<Self> {
        if expr.is_empty() {
            return None;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        // r#"15*16+2-9^2/3^{19+2.3}+(5.3+1)"#
        dbg!(ExpressionBuffer::parse_raw(
            r#"15*16+2-9^2/3^{19-(-5+1)+2.3}+(5.3+1)"#
        ));
    }

    #[test]
    fn test_expr_parser_func1() {
        dbg!(ExpressionBuffer::parse_raw(
            r#"5+\frac{2^5+\frac{1}{2}+\sqrt{2}{4}}{3}+5*3"#
        ));
    }

    #[test]
    fn test_expr_parser_func2() {
        dbg!(ExpressionBuffer::parse_raw(
            r#"1/\frac{\lg_{5}}{\log_{3}{8}}^5+\ln_{\sqrt{2}}"#
        ));
    }

    #[test]
    fn test_expr_parser_func3() {
        dbg!(ExpressionBuffer::parse_raw(r#"\sin{\sqrt{3}}"#));
    }
}
