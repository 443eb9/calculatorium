use crate::{
    latex::*,
    math::{
        func::decl::{get_phantom_function, get_phantom_operator},
        symbol::{BracketState, Constant, Number},
        ErrorLocation, ExpressionElement, FromRawExpr, Function, IntoRawExpr, LaTexParsingError,
        LaTexParsingResult, MathElement, ParsingErrorType,
    },
    utils::BracketStack,
    DecimalScalar,
};

#[derive(Debug)]
pub struct ExpressionBuffer {
    pub expr: Vec<MathElement>,
}

impl FromRawExpr for ExpressionBuffer {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self> {
        if expr.is_empty() {
            return Err(LaTexParsingError::new(
                ErrorLocation::Raw(0),
                ParsingErrorType::EmptyInput,
            ));
        }

        let mut expr_buffer = Vec::new();
        let mut univariate_funcs = Vec::new();

        // The start index of a number: 23, 2.3
        let mut number_start = -1;
        // The start index of a function subexpression, without the start curly bracket
        let mut func_subexpr_start = -1;
        // The depth of a function subexpression
        let mut func_subexpr_start_depth = 0;
        // The start index of a user-defined subexpression, without the start parenthese
        let mut user_subexpr_start = -1;
        // The depth of a user-defined subexpression
        let mut user_subexpr_start_depth = 0;
        // The start index of a function definition
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
                    ErrorLocation::Raw(i as u32),
                    ParsingErrorType::InvalidBracketStructure,
                ));
            }

            // Function Subexpressions
            if func_subexpr_start != -1
                && ((c == CURLY_BRACKET_R && curly_brackets.depth() == func_subexpr_start_depth)
                    || c == WHITESPACE)
            {
                expr_buffer.push(Some(MathElement::Expression(
                    ExpressionBuffer::parse_raw_with_base_index(
                        &expr[func_subexpr_start as usize..i],
                        func_subexpr_start as u32,
                    )?,
                )));
                func_subexpr_start = -1;
                continue;
            }

            // User Subexpressions
            if user_subexpr_start != -1
                && c == PARENTHESES_R
                && parentheses.depth() == user_subexpr_start_depth
            {
                expr_buffer.push(Some(MathElement::Parentheses(BracketState::Open)));
                expr_buffer.extend(
                    ExpressionBuffer::parse_raw_with_base_index(
                        &expr[user_subexpr_start as usize..i],
                        user_subexpr_start as u32,
                    )?
                    .expr
                    .into_iter()
                    .map(|e| Some(e)),
                );
                expr_buffer.push(Some(MathElement::Parentheses(BracketState::Close)));
                user_subexpr_start = -1;
                continue;
            }

            // Functions
            if func_def_start != -1 && (c == CURLY_BRACKET_L || c == WHITESPACE) {
                let mut f_name = &expr[func_def_start as usize..i];
                let mut optional_param = handle_optional_params(f_name);

                // Root has a optinal parameter that is wrapped around []
                if f_name.starts_with(ROOT) && f_name != ROOT {
                    optional_param = Some(
                        Number::parse_raw(&f_name[f_name.len() - ROOT.len() + 2..f_name.len() - 1])
                            .ok_or_else(|| {
                                LaTexParsingError::new(
                                    ErrorLocation::Raw(func_def_start as u32),
                                    ParsingErrorType::InvalidFunctionInvocation(f_name.to_string()),
                                )
                            })?,
                    );
                    f_name = ROOT;
                }

                let f = get_phantom_function(f_name).ok_or_else(|| {
                    LaTexParsingError::new(
                        ErrorLocation::Raw(func_def_start as u32 - 1),
                        ParsingErrorType::InvalidFunctionName(f_name.to_string()),
                    )
                })?;

                expr_buffer.push(Some(MathElement::PhantomFunction(f)));
                if let Some(opt) = optional_param {
                    expr_buffer.push(Some(MathElement::Expression(ExpressionBuffer {
                        expr: vec![MathElement::Number(opt)],
                    })));
                }
                func_def_start = -1;

                if c == WHITESPACE {
                    univariate_funcs.push(expr_buffer.len() - 1);
                    continue;
                }
            }

            if number_start != -1 {
                if !(c.is_digit(10) || c == '.') {
                    // Real Numbers
                    if let Some(scalar) = Number::parse_raw(&expr[number_start as usize..i]) {
                        expr_buffer.push(Some(MathElement::Number(scalar)));
                        number_start = -1;
                    } else {
                        return Err(LaTexParsingError::new(
                            ErrorLocation::Raw(number_start as u32),
                            ParsingErrorType::InvalidNumber(
                                expr[number_start as usize..i].to_string(),
                            ),
                        ));
                    }
                } else {
                    continue;
                }
            }

            if func_subexpr_start != -1 || user_subexpr_start != -1 || func_def_start != -1 {
                continue;
            }

            if c == CURLY_BRACKET_L {
                func_subexpr_start = i as i32 + 1;
                func_subexpr_start_depth = curly_brackets.depth() - 1;
                continue;
            }

            if c == PARENTHESES_L {
                user_subexpr_start = i as i32 + 1;
                user_subexpr_start_depth = parentheses.depth() - 1;
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

            // Constants

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
                    expr_buffer.push(Some(MathElement::Number(Number::Integer(0))));
                }

                expr_buffer.push(Some(MathElement::PhantomOperator(
                    get_phantom_operator(c.to_string().as_str()).unwrap(),
                )));
                continue;
            }

            return Err(LaTexParsingError::new(
                ErrorLocation::Raw(i as u32),
                ParsingErrorType::Unknown,
            ));
        }

        if number_start != -1 {
            if let Some(scalar) = Number::parse_raw(&expr[number_start as usize..]) {
                expr_buffer.push(Some(MathElement::Number(scalar)));
            } else {
                return Err(LaTexParsingError::new(
                    ErrorLocation::Raw(number_start as u32),
                    ParsingErrorType::InvalidNumber(expr[number_start as usize..].to_string()),
                ));
            }
        }

        // Handle univariate functions
        univariate_funcs.into_iter().for_each(|fn_idx| {
            let param = expr_buffer[fn_idx + 1].as_ref().unwrap();
            match param {
                MathElement::Parentheses(p) => {
                    let mut j = fn_idx + 2;
                    let mut stack = BracketStack::default();
                    stack.push(*p);
                    expr_buffer[fn_idx + 1].take();

                    loop {
                        if let Some(MathElement::Parentheses(p)) = expr_buffer[j] {
                            stack.push(p);
                            if stack.depth() == 0 {
                                expr_buffer[j].take();
                                break;
                            }
                        }
                        j += 1;
                    }

                    expr_buffer[fn_idx + 1] = Some(MathElement::Expression(ExpressionBuffer {
                        expr: expr_buffer[fn_idx + 2..j]
                            .iter_mut()
                            .map(|e| e.take().unwrap())
                            .collect(),
                    }));
                }
                MathElement::PhantomFunction(phf) => {
                    let n = phf.num_params() as usize;
                    expr_buffer[fn_idx + 1] = Some(MathElement::Expression(ExpressionBuffer {
                        expr: expr_buffer[fn_idx + 1..fn_idx + 2 + n]
                            .iter_mut()
                            .map(|e| e.take().unwrap())
                            .collect(),
                    }))
                }
                _ => {}
            }
        });

        Ok(Self {
            expr: expr_buffer.into_iter().filter_map(|e| e).collect(),
        })
    }
}

fn handle_optional_params(f_name: &str) -> Option<Number> {
    match f_name {
        ROOT => Some(Number::Integer(2)),
        LG => Some(Number::Integer(10)),
        LN => Some(Number::Constant(Constant::E)),
        _ => None,
    }
}

impl ExpressionBuffer {
    fn to_postfix(self) -> LaTexParsingResult<Vec<MathElement>> {
        if self.expr.is_empty() {
            return Err(LaTexParsingError::new(
                ErrorLocation::Tokenized(0),
                ParsingErrorType::EmptyInput,
            ));
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
                        ErrorLocation::Tokenized(i as u32),
                        ParsingErrorType::InvalidFunctionInvocation(format!("{:?}", phf)),
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
                // Only exists when the prior element is `Power`
                MathElement::Expression(e) => buffer.push(MathElement::Expression(e)),
                _ => {
                    return Err(LaTexParsingError::new(
                        ErrorLocation::Tokenized(i as u32),
                        ParsingErrorType::Unknown,
                    ));
                }
            }
        }

        let mut fn_stack = Vec::new();
        let mut num_stack = Vec::new();

        for (i, elem) in buffer.drain(..).enumerate() {
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
                // Only exists when the prior element is `Power`
                MathElement::Expression(e) => num_stack.push(MathElement::Expression(e)),
                _ => {
                    return Err(LaTexParsingError::new(
                        ErrorLocation::Tokenized(i as u32),
                        ParsingErrorType::Unknown,
                    ));
                }
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

impl Function for ExpresssionTree {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.root.approximate()
    }
}

impl IntoRawExpr for ExpresssionTree {
    fn assemble(&self) -> String {
        self.root.assemble()
    }
}

impl ExpresssionTree {
    fn from_postfix(expr: Vec<MathElement>) -> LaTexParsingResult<Self> {
        if expr.is_empty() {
            return Err(LaTexParsingError::new(
                ErrorLocation::Tokenized(0),
                ParsingErrorType::EmptyInput,
            ));
        }

        let mut expr = expr.into_iter().rev().collect::<Vec<_>>();
        let mut tree_buffer = Vec::new();

        while let Some(elem) = expr.pop() {
            match elem {
                MathElement::Number(n) => tree_buffer.push(ExpressionElement::Number(n)),
                MathElement::Function(f) => tree_buffer.push(ExpressionElement::Function(f)),
                MathElement::PhantomOperator(pho) => {
                    let mut params = vec![tree_buffer.pop(), tree_buffer.pop()];
                    params.reverse();
                    tree_buffer.push(ExpressionElement::Operator(pho.solidify(params)));
                }
                // Only exists when the operator is `Power`
                MathElement::Expression(e) => {
                    tree_buffer.push(ExpresssionTree::from_postfix(e.to_postfix()?)?.root)
                }
                _ => unreachable!(),
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
    fn test_univariate_funcs() {
        assert_eq!(
            ExpresssionTree::parse_raw(r#"\sin \frac{4}{7}"#)
                .unwrap()
                .approximate() as f32,
            0.5408342134
        );
        assert_eq!(
            ExpresssionTree::parse_raw(r#"\cos (5-1)*2"#)
                .unwrap()
                .approximate() as f32,
            -1.307287242
        );
    }

    #[test]
    fn test_expr_approximation() {
        assert_eq!(
            ExpresssionTree::parse_raw(r#"15*16+2-9^2/3^{19-(-5+1)+2.3}+(5.3+1)"#)
                .unwrap()
                .approximate() as f32,
            248.3
        );
        assert_eq!(
            ExpresssionTree::parse_raw(r#"\sin{\sqrt[4]{3}}"#)
                .unwrap()
                .approximate() as f32,
            0.9677333034
        );
        assert_eq!(
            ExpresssionTree::parse_raw(r#"5+\frac{2^5+\frac{1}{2}+\sqrt{58}}{3}+5*3"#)
                .unwrap()
                .approximate() as f32,
            33.37192437
        );
        assert_eq!(
            ExpresssionTree::parse_raw(r#"1/\frac{\lg{5}}{\log_{3}{8}}^5+\ln{\sqrt{2}}"#)
                .unwrap()
                .approximate() as f32,
            145.9657673
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
