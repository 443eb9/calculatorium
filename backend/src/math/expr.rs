use crate::{
    func::decl::{get_phantom_function, FromRawExpr, IntoRawExpr, MathFunction},
    latex::*,
    math::symbol::{BracketState, Number},
    utils::BracketStack,
};

use super::{
    MathElement, LaTexParsingError, LaTexParsingErrorType, LaTexParsingResult, ExpressionElement,
};

#[derive(Debug)]
pub struct ExpressionBuffer {
    pub expr: Vec<MathElement>,
}

impl FromRawExpr for ExpressionBuffer {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        let mut expr_buffer = Vec::new();

        let mut number_start = -1;
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
                expr_buffer.push(MathElement::Expression(
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
                expr_buffer.push(MathElement::Expression(
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
                expr_buffer.push(MathElement::PhantomFunction(
                    get_phantom_function(&expr[func_def_start as usize..i]).ok_or_else(|| {
                        LaTexParsingError::new(
                            func_def_start as u32 - 1,
                            LaTexParsingErrorType::InvalidFunctionName(
                                (&expr[func_def_start as usize..i]).to_string(),
                            ),
                        )
                    })?,
                ));
                func_def_start = -1;
            }

            if number_start != -1 {
                if !(c.is_digit(10) || c == '.') {
                    // Real Numbers
                    if let Some(scalar) = Number::parse_raw(&expr[number_start as usize..i]) {
                        expr_buffer.push(MathElement::Number(scalar));
                        number_start = -1;
                    } else {
                        return Err(LaTexParsingError::new(
                            number_start as u32,
                            LaTexParsingErrorType::InvalidNumber(
                                expr[number_start as usize..i].to_string(),
                            ),
                        ));
                    }
                } else {
                    continue;
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

            if c.is_digit(10) || c == '.' {
                number_start = i as i32;
                continue;
            }

            // Custom Variables
            if c.is_ascii_lowercase() {
                todo!("parse custom variables");
                // continue;
            }

            // Operators
            if matches!(
                c.to_string().as_str(),
                ADD | SUBTRACT | MULTIPLY | DIVIDE | SUPER_SCRIPT
            ) {
                if matches!(c.to_string().as_str(), ADD | SUBTRACT) && expr_buffer.is_empty() {
                    expr_buffer.push(MathElement::Number(Number::Integer(0)));
                }

                expr_buffer.push(MathElement::PhantomFunction(
                    get_phantom_function(c.to_string().as_str()).unwrap(),
                ));
                continue;
            }

            return Err(LaTexParsingError::new(
                i as u32,
                LaTexParsingErrorType::Unknown,
            ));
        }

        if number_start != -1 {
            if let Some(scalar) = Number::parse_raw(&expr[number_start as usize..]) {
                expr_buffer.push(MathElement::Number(scalar));
            } else {
                return Err(LaTexParsingError::new(
                    number_start as u32,
                    LaTexParsingErrorType::InvalidNumber(expr[number_start as usize..].to_string()),
                ));
            }
        }

        Ok(Self { expr: expr_buffer })
    }
}

impl IntoRawExpr for ExpressionBuffer {
    fn assemble(&self) -> String {
        self.expr
            .iter()
            .map(|elem| elem.assemble().chars().collect::<Vec<_>>())
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
pub struct ExpresssionTree {
    root: ExpressionElement,
}

impl FromRawExpr for ExpresssionTree {
    /// Full-featured LaTex parser.
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        let mut raw_buffer = ExpressionBuffer::parse_raw(expr)?
            .expr
            .into_iter()
            .map(|e| Some(e))
            .collect::<Vec<_>>();

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        assert_eq!(
            ExpressionBuffer::parse_raw(r#"15*16+2-9^2/3^{19-(-5+1)+2.3}+(5.3+1)"#)
                .unwrap()
                .assemble(),
            "15Multiply16Add2Subtract9Power2Divide3Power19Subtract0Subtract5Add1Add2.3Add5.3Add1"
        );
    }

    #[test]
    fn test_expr_parser_func1() {
        assert_eq!(
            ExpressionBuffer::parse_raw(r#"5+\frac{2^5+\frac{1}{2}+\sqrt{2}{4}}{3}+5*3"#)
                .unwrap()
                .assemble(),
            "5AddFraction2Power5AddFraction12AddRoot243Add5Multiply3"
        );
    }

    #[test]
    fn test_expr_parser_func2() {
        assert_eq!(
            ExpressionBuffer::parse_raw(r#"1/\frac{\lg_{5}}{\log_{3}{8}}^5+\ln_{\sqrt{2}}"#)
                .unwrap()
                .assemble(),
            "1DivideFractionLg5Log38Power5AddLnRoot2"
        );
    }

    #[test]
    fn test_expr_parser_func3() {
        assert_eq!(
            ExpressionBuffer::parse_raw(r#"\sin{\sqrt{3}}"#)
                .unwrap()
                .assemble(),
            "SinRoot3"
        );
    }

    #[test]
    fn test_parsing_err() {
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"{}())()"#));
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"2_3*6"#));
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"2-\frad{3}{4}"#));
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"1+3.."#));
        let _ = dbg!(ExpressionBuffer::parse_raw(r#"2+(5/0..9)"#));
    }
}
