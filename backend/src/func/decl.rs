use std::fmt::Debug;

use super::PhantomFunction;

use crate::{
    func::{Function, Operator, PhantomOperator},
    latex::*,
    math::{ExpressionElement, LaTexParsingError, LaTexParsingResult},
};

use calculatorium_macros::{AsPhantomFunction, AsPhantomOperator, FromExpr, IntoRawExpr};

pub trait FromRawExpr {
    fn parse_raw(expr: &str) -> LaTexParsingResult<Self>
    where
        Self: Sized;

    fn parse_raw_with_base_index(expr: &str, base: u32) -> LaTexParsingResult<Self>
    where
        Self: Sized,
    {
        Self::parse_raw(expr).map_err(|e| LaTexParsingError::new(e.at + base, e.ty))
    }
}

pub trait IntoRawExpr {
    fn assemble(&self) -> String;
}

pub trait Prioritizable {
    /// The higher, the earlier it is evaluated.
    /// - Add/Subtract/Binary/Modulo Operations 1
    /// - Multiply/Divide 5
    /// - Functions (Power, Log, Sin etc) 10
    fn priority(&self) -> u32;
}

pub trait FromExpr {
    fn convert(expr: Vec<Option<ExpressionElement>>) -> Self
    where
        Self: Sized;
}

macro_rules! register_phantom_functions {
    ($($fn_name: pat, $phfn_ty: ty),*) => {
        pub fn get_phantom_function(name: &str) -> Option<Box<dyn PhantomFunction>> {
            match name {
                $($fn_name => Some(Box::new(<$phfn_ty>::default())),)*
                _ => None
            }
        }
    };
}

macro_rules! register_phantom_operators {
    ($($op_name: pat, $phop_ty: ty),*) => {
        pub fn get_phantom_operator(name: &str) -> Option<Box<dyn PhantomOperator>> {
            match name {
                $($op_name => Some(Box::new(<$phop_ty>::default())),)*
                _ => None
            }
        }
    };
}

macro_rules! define_operator {
    ($priority: literal, $op_ty: ident, $op_name: expr, $($field: ident),*) => {
        #[derive(Debug, FromExpr, AsPhantomOperator)]
        #[priority($priority)]
        pub struct $op_ty {
            $($field: ExpressionElement,)*
        }

        impl $op_ty {
            pub const LATEX_SYMBOL: &'static str = $op_name;
        }

        impl Prioritizable for $op_ty {
            fn priority(&self) -> u32 {
                $priority
            }
        }
    };
}

macro_rules! define_function {
    ($fn_ty: ident, $fn_name: expr, $($field: ident),*) => {
        #[derive(Debug, FromExpr, IntoRawExpr, AsPhantomFunction)]
        pub struct $fn_ty {
            $($field: ExpressionElement,)*
        }

        impl $fn_ty {
            pub const LATEX_SYMBOL: &'static str = $fn_name;
        }
    };
}

macro_rules! impl_into_raw_expr_op {
    ($op_ty: ident, $symbol: expr) => {
        impl IntoRawExpr for $op_ty {
            fn assemble(&self) -> String {
                if self.lhs.priority() < self.priority() {
                    format!(
                        "{}{}({})",
                        self.rhs.assemble(),
                        $symbol,
                        self.lhs.assemble()
                    )
                } else {
                    format!("{}{}{}", self.lhs.assemble(), $symbol, self.rhs.assemble())
                }
            }
        }
    };
}

impl IntoRawExpr for Power {
    fn assemble(&self) -> String {
        format!("{}^{{{}}}", self.base.assemble(), self.exp.assemble())
    }
}

define_operator!(1, Add, ADD, lhs, rhs);
define_operator!(1, Subtract, SUBTRACT, lhs, rhs);
define_operator!(5, Multiply, MULTIPLY, lhs, rhs);
define_operator!(5, Divide, DIVIDE, lhs, rhs);
define_operator!(10, Power, SUPER_SCRIPT, base, exp);

impl_into_raw_expr_op!(Add, ADD);
impl_into_raw_expr_op!(Subtract, SUBTRACT);
impl_into_raw_expr_op!(Multiply, MULTIPLY);
impl_into_raw_expr_op!(Divide, DIVIDE);

define_function!(Fraction, FRAC, num, den);
define_function!(Root, ROOT, rad, deg);

define_function!(Log, LOG, base, anti);

define_function!(Sin, SIN, x);
define_function!(Cos, COS, x);
define_function!(Tan, TAN, x);

#[rustfmt::skip]
register_phantom_functions!(
    FRAC, PhantomFraction,
    ROOT, PhantomRoot,
    
    LOG | LG | LN, PhantomLog,

    SIN, PhantomSin,
    COS, PhantomCos,
    TAN, PhantomTan
);

#[rustfmt::skip]
register_phantom_operators!(
    ADD, PhantomAdd,
    SUBTRACT, PhantomSubtract,
    MULTIPLY, PhantomMultiply,
    DIVIDE, PhantomDivide,
    SUPER_SCRIPT, PhantomPower
);

// #[cfg(test)]
// mod test {
//     use crate::math::symbol::Number;

//     use super::*;

//     #[test]
//     fn test_into_raw_expr() {
//         assert_eq!(
//             Add {
//                 lhs: ExpressionElement::Number(Number::Integer(1)),
//                 rhs: ExpressionElement::Function(Box::new(Subtract {
//                     lhs: ExpressionElement::Number(Number::Integer(2)),
//                     rhs: ExpressionElement::Number(Number::Decimal(3.8))
//                 })),
//             }
//             .assemble(),
//             "1+2-3.8"
//         );

//         // When the operator precedence of the lhs is lower than itself,
//         // wrap the contents of the lhs with parentheses and place it on the right side.
//         assert_eq!(
//             Fraction {
//                 num: ExpressionElement::Function(Box::new(Sin {
//                     x: ExpressionElement::Number(Number::Integer(3))
//                 })),
//                 den: ExpressionElement::Function(Box::new(Multiply {
//                     lhs: ExpressionElement::Function(Box::new(Add {
//                         lhs: ExpressionElement::Number(Number::Integer(5)),
//                         rhs: ExpressionElement::Number(Number::Integer(7))
//                     })),
//                     rhs: ExpressionElement::Number(Number::Decimal(6.5)),
//                 })),
//             }
//             .assemble(),
//             "\\frac{\\sin{3}}{6.5*(5+7)}"
//         );
//     }
// }
