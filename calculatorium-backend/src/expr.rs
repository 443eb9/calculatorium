use crate::{
    func::decl::{get_phantom_function, FromRawExpr},
    latex::{
        BracketState, LaTexElement, Operator, CURLY_BRACKET_L, CURLY_BRACKET_R, FUNC_BEGIN,
        PARENTHESES_L, PARENTHESES_R,
    },
    symbol::RealNumber,
    utils::BracketStack,
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

        Self::tokenize(expr).and_then(|expr| Self::parse(expr))
    }
}

impl LaTexExpression {
    fn tokenize(expr: &str) -> Option<Self> {
        let mut symbols = Vec::new();
        let mut digit_start = -1;
        let mut sub_expr_start = -1;
        let mut sub_expr_start_depth = 0;
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

            // Sub Expressions
            if sub_expr_start != -1 {
                if c == CURLY_BRACKET_R && curly_brackets.depth() == sub_expr_start_depth {
                    if let Some(sub_expr) =
                        LaTexExpression::parse_raw(&expr[sub_expr_start as usize..i])
                    {
                        symbols.push(LaTexElement::Expression(sub_expr));
                        sub_expr_start = -1;
                    } else {
                        return None;
                    }
                }
                continue;
            }

            // Functions
            if func_def_start != -1 {
                if c == CURLY_BRACKET_L {
                    if let Some(ph_func) = get_phantom_function(&expr[func_def_start as usize..i]) {
                        symbols.push(LaTexElement::PhantomFunction(ph_func));
                        func_def_start = -1;
                    } else {
                        return None;
                    }
                } else {
                    continue;
                }
            }

            if c == CURLY_BRACKET_L {
                sub_expr_start = i as i32 + 1;
                sub_expr_start_depth = curly_brackets.depth() - 1;
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
                if let Some(scalar) = RealNumber::parse_raw(&expr[digit_start as usize..i]) {
                    symbols.push(LaTexElement::RealNumber(scalar));
                    digit_start = -1;
                }
            }

            // Operators
            if let Some(op) = Operator::parse_raw(&c.to_string()) {
                symbols.push(LaTexElement::Operator(op));
                continue;
            }

            // Parentheses
            if c == PARENTHESES_L {
                symbols.push(LaTexElement::Parentheses(BracketState::Open));
                continue;
            }
            if c == PARENTHESES_R {
                symbols.push(LaTexElement::Parentheses(BracketState::Close));
                continue;
            }

            return None;
        }

        if digit_start != -1 {
            if let Some(scalar) = RealNumber::parse_raw(&expr[digit_start as usize..]) {
                symbols.push(LaTexElement::RealNumber(scalar));
            } else {
                return None;
            }
        }

        Some(Self { raw: symbols })
    }

    fn parse(mut expr: Self) -> Option<Self> {
        Some(expr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        dbg!(LaTexExpression::parse_raw(
            r#"15*16+2-9^2/3^{19+2.3}+(5.3+1)"#
        ));
    }

    #[test]
    fn test_expr_parser_func() {
        dbg!(LaTexExpression::parse_raw(
            r#"5+\frac{2^5+\frac{1}{2}}{3}+5*3"#
        ));
    }
}
