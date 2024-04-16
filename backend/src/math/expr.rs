use crate::{
    func::decl::{get_phantom_function, FromRawExpr, MathFunction},
    latex::*,
    math::symbol::{BracketState, Number},
    utils::BracketStack,
};

use super::{
    symbol::MathSymbol, ExpressionElement, LaTexParsingError, LaTexParsingErrorType,
    LaTexParsingResult,
};

#[derive(Debug)]
pub struct ExpressionBuffer {
    pub expr: Vec<ExpressionElement>,
}

impl FromRawExpr for ExpressionBuffer {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
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
                return Err(LaTexParsingError::new(
                    i as u32,
                    LaTexParsingErrorType::InvalidBracketStructure,
                ));
            }

            // Function Sub Expressions
            if func_sub_expr_start != -1
                && c == CURLY_BRACKET_R
                && curly_brackets.depth() == func_sub_expr_start_depth
            {
                expr_buffer.push(ExpressionElement::Expression(
                    ExpressionBuffer::parse_raw_with_base_index(
                        &expr[func_sub_expr_start as usize..i],
                        func_sub_expr_start as u32,
                    )?,
                ));
                func_sub_expr_start = -1;
                continue;
            }

            // User Sub Expressions
            if user_sub_expr_start != -1
                && c == PARENTHESES_R
                && parentheses.depth() == user_sub_expr_start_depth
            {
                expr_buffer.push(ExpressionElement::Expression(
                    ExpressionBuffer::parse_raw_with_base_index(
                        &expr[user_sub_expr_start as usize..i],
                        user_sub_expr_start as u32,
                    )?,
                ));
                user_sub_expr_start = -1;
                continue;
            }

            // Functions
            if func_def_start != -1 && c == CURLY_BRACKET_L {
                expr_buffer.push(ExpressionElement::Function(MathFunction::Phantom(
                    get_phantom_function(&expr[func_def_start as usize..i]).ok_or_else(|| {
                        LaTexParsingError::new(
                            func_def_start as u32 - 1,
                            LaTexParsingErrorType::InvalidFunctionName(
                                (&expr[func_def_start as usize..i]).to_string(),
                            ),
                        )
                    })?,
                )));
                func_def_start = -1;
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

            return Err(LaTexParsingError::new(
                i as u32,
                LaTexParsingErrorType::Unknown,
            ));
        }

        if digit_start != -1 {
            if let Some(scalar) = Number::parse_raw(&expr[digit_start as usize..]) {
                expr_buffer.push(ExpressionElement::Symbol(MathSymbol::Number(scalar)));
            } else {
                return Err(LaTexParsingError::new(
                    digit_start as u32,
                    LaTexParsingErrorType::InvalidNumber(expr[digit_start as usize..].to_string()),
                ));
            }
        }

        Ok(Self { expr: expr_buffer })
    }
}

#[derive(Debug)]
pub struct LaTexExpression {
    root: MathFunction,
}

impl FromRawExpr for LaTexExpression {
    /// Full-featured LaTex parser.
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        // r#"15*16+2-9^2/3^{19+2.3}+(5.3+1)"#
        dbg!(ExpressionBuffer::parse_raw(r#"15*16+2-9^2/3^{19-(-5+1)+2.3}+(5.3+1)"#).unwrap());
    }

    #[test]
    fn test_expr_parser_func1() {
        dbg!(
            ExpressionBuffer::parse_raw(r#"5+\frac{2^5+\frac{1}{2}+\sqrt{2}{4}}{3}+5*3"#).unwrap()
        );
    }

    #[test]
    fn test_expr_parser_func2() {
        dbg!(
            ExpressionBuffer::parse_raw(r#"1/\frac{\lg_{5}}{\log_{3}{8}}^5+\ln_{\sqrt{2}}"#)
                .unwrap()
        );
    }

    #[test]
    fn test_expr_parser_func3() {
        dbg!(ExpressionBuffer::parse_raw(r#"\sin{\sqrt{3}}"#).unwrap());
    }

    #[test]
    fn test_parsing_err() {
        // let _ = dbg!(ExpressionBuffer::parse_raw(r#"{}())()"#));
        // let _ = dbg!(ExpressionBuffer::parse_raw(r#"2_3*6"#));
        // let _ = dbg!(ExpressionBuffer::parse_raw(r#"2-\frad{3}{4}"#));
        // let _ = dbg!(ExpressionBuffer::parse_raw(r#"1+3."#));
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"2+(5/9.)"#));
    }
}
