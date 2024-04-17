use crate::{
    func::decl::{get_phantom_function, get_phantom_operator, FromRawExpr, IntoRawExpr},
    latex::*,
    math::symbol::{BracketState, Number},
    utils::BracketStack,
};

use super::{
    ExpressionElement, LaTexParsingError, LaTexParsingErrorType, LaTexParsingResult, MathElement,
    MathFunctionType,
};

#[derive(Debug)]
pub struct ExpressionBuffer {
    pub expr: Vec<MathElement>,
}

impl FromRawExpr for ExpressionBuffer {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        if expr.is_empty() {
            return Err(LaTexParsingError::new(0, LaTexParsingErrorType::EmptyInput));
        }

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
                expr_buffer.push(MathElement::Parentheses(BracketState::Open));
                expr_buffer.extend(
                    ExpressionBuffer::parse_raw_with_base_index(
                        &expr[user_sub_expr_start as usize..i],
                        user_sub_expr_start as u32,
                    )?
                    .expr,
                );
                expr_buffer.push(MathElement::Parentheses(BracketState::Close));
                user_sub_expr_start = -1;
                continue;
            }

            // Functions
            if func_def_start != -1 && c == CURLY_BRACKET_L {
                let f =
                    get_phantom_function(&expr[func_def_start as usize..i]).ok_or_else(|| {
                        LaTexParsingError::new(
                            func_def_start as u32 - 1,
                            LaTexParsingErrorType::InvalidFunctionName(
                                (&expr[func_def_start as usize..i]).to_string(),
                            ),
                        )
                    })?;

                expr_buffer.push(MathElement::PhantomFunction(f));
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

                expr_buffer.push(MathElement::PhantomOperator(
                    get_phantom_operator(c.to_string().as_str()).unwrap(),
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

impl ExpressionBuffer {
    pub fn to_postfix(self) -> LaTexParsingResult<Vec<MathElement>> {
        if self.expr.is_empty() {
            return Err(LaTexParsingError::new(0, LaTexParsingErrorType::EmptyInput));
        }

        let mut raw_buffer = self.expr.into_iter().map(|e| Some(e)).collect::<Vec<_>>();

        let mut buffer = Vec::new();

        // Solidify all the PhantomFunctions.
        for i in 0..raw_buffer.len() {
            if raw_buffer[i].is_none() {
                continue;
            }

            match raw_buffer[i].take().unwrap() {
                MathElement::Number(n) => buffer.push(MathElement::Number(n)),
                MathElement::Parentheses(p) => buffer.push(MathElement::Parentheses(p)),
                MathElement::PhantomFunction(phf) => {
                    let n = phf.num_params() as usize;
                    let mut params = Vec::with_capacity(n);

                    let err_template = LaTexParsingError::new(
                        i as u32,
                        LaTexParsingErrorType::InvalidFunctionInvocation(format!("{:?}", phf)),
                    );

                    for param in &mut raw_buffer[i + 1..i + 1 + n] {
                        let param = param.take().ok_or_else(|| err_template.clone())?;

                        let converted = match param {
                            MathElement::Number(n) => ExpressionElement::Number(n),
                            MathElement::Expression(e) => {
                                ExpresssionTree::from_postfix(e.to_postfix()?)?.root
                            }
                            _ => return Err(err_template),
                        };

                        params.push(Some(converted));
                    }

                    buffer.push(MathElement::Function(phf.solidify(params)));
                }
                MathElement::PhantomOperator(pho) => buffer.push(MathElement::PhantomOperator(pho)),
                MathElement::Function(_) => todo!(),
                MathElement::Operator(_) => todo!(),
                MathElement::Expression(_) => todo!(),
            }
        }

        let mut fn_stack = Vec::new();
        let mut num_stack = Vec::new();

        for elem in buffer.drain(..) {
            match elem {
                MathElement::Number(n) => num_stack.push(MathElement::Number(n)),
                MathElement::Parentheses(p) => match p {
                    BracketState::Open => fn_stack.push(MathElement::Parentheses(p)),
                    BracketState::Close => loop {
                        let elem = fn_stack.pop().unwrap();
                        if let MathElement::Parentheses(p) = &elem {
                            if *p == BracketState::Open {
                                break;
                            }
                        }
                        num_stack.push(elem);
                    },
                },
                MathElement::Function(f) => num_stack.push(MathElement::Function(f)),
                MathElement::PhantomOperator(pho) => {
                    if fn_stack.is_empty()
                        || fn_stack.last().is_some_and(|f| match f {
                            MathElement::PhantomOperator(pho1) => pho.priority() > pho1.priority(),
                            MathElement::Parentheses(_) => true,
                            _ => unreachable!(),
                        })
                    {
                        fn_stack.push(MathElement::PhantomOperator(pho));
                    } else {
                        while fn_stack.last().is_some_and(|f| match f {
                            MathElement::PhantomOperator(pho1) => pho.priority() <= pho1.priority(),
                            MathElement::Parentheses(_) => false,
                            _ => unreachable!(),
                        }) {
                            num_stack.push(fn_stack.pop().unwrap());
                        }
                        fn_stack.push(MathElement::PhantomOperator(pho));
                    }
                }
                MathElement::Operator(_) => todo!(),
                MathElement::PhantomFunction(_) => todo!(),
                MathElement::Expression(_) => todo!(),
            }
        }

        num_stack.extend(fn_stack.into_iter().rev());

        Ok(num_stack)
    }
}

#[derive(Debug)]
pub struct ExpresssionTree {
    root: ExpressionElement,
}

impl FromRawExpr for ExpresssionTree {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        ExpresssionTree::from_postfix(ExpressionBuffer::parse_raw(expr)?.to_postfix()?)
    }
}

impl ExpresssionTree {
    pub fn from_postfix(expr: Vec<MathElement>) -> LaTexParsingResult<Self> {
        if expr.is_empty() {
            return Err(LaTexParsingError::new(0, LaTexParsingErrorType::EmptyInput));
        }

        let mut expr = expr.into_iter().rev().collect::<Vec<_>>();
        let mut tree_buffer = Vec::new();

        while let Some(elem) = expr.pop() {
            match elem {
                MathElement::Number(n) => tree_buffer.push(ExpressionElement::Number(n)),
                MathElement::Parentheses(_) => todo!(),
                MathElement::Function(f) => tree_buffer.push(ExpressionElement::Function(f)),
                MathElement::Operator(_) => todo!(),
                MathElement::PhantomFunction(_) => todo!(),
                MathElement::PhantomOperator(pho) => {
                    let params = vec![tree_buffer.pop(), tree_buffer.pop()];
                    tree_buffer.push(ExpressionElement::Operator(pho.solidify(params)));
                }
                MathElement::Expression(_) => todo!(),
            }
        }

        let root = tree_buffer.drain(..).next().unwrap();

        Ok(Self { root })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser_simple() {
        dbg!(ExpresssionTree::from_postfix(
            ExpressionBuffer::parse_raw(r#"15*16+2-9^2/3^{19-(-5+1)+2.3}+(5.3+1)"#)
                .unwrap()
                .to_postfix()
                .unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_expr_parser_func1() {
        dbg!(ExpresssionTree::from_postfix(
            ExpressionBuffer::parse_raw(r#"5+\frac{2^5+\frac{1}{2}+\sqrt{2}{4}}{3}+5*3"#)
                .unwrap()
                .to_postfix()
                .unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_expr_parser_func2() {
        // dbg!(ExpresssionTree::from_postfix(
        //     ExpressionBuffer::parse_raw(r#"1/\frac{\lg_{5}}{\log_{3}{8}}^5+\ln_{\sqrt{2}}"#)
        //         .unwrap()
        //         .to_postfix()
        //         .unwrap()
        // )
        // .unwrap());
        dbg!(ExpresssionTree::from_postfix(
            ExpressionBuffer::parse_raw(r#"\frac{\lg_{5}}{\log_{3}{8}}^5"#)
                .unwrap()
                .to_postfix()
                .unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_expr_parser_func3() {
        dbg!(ExpresssionTree::from_postfix(
            ExpressionBuffer::parse_raw(r#"\sin{\sqrt{2}{3}}"#)
                .unwrap()
                .to_postfix()
                .unwrap()
        )
        .unwrap());
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
