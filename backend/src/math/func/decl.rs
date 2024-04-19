use std::fmt::Debug;

use crate::{
    latex::*,
    math::{
        ExpressionElement, FromExpr, Function, IntoRawExpr, MathElement, PhantomFunction,
        PhantomOperator, Prioritizable,
    },
    DecimalScalar,
};

use calculatorium_macros::{AsPhantomFunction, AsPhantomOperator, FromExpr, IntoRawExpr};

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
define_function!(Cot, COT, x);
define_function!(Sec, SEC, x);
define_function!(Csc, CSC, x);

define_function!(Arcsin, ARCSIN, x);
define_function!(Arccos, ARCCOS, x);
define_function!(Arctan, ARCTAN, x);
define_function!(Arccot, ARCCOT, x);
define_function!(Arcsec, ARCSEC, x);
define_function!(Arccsc, ARCCSC, x);

define_function!(Sinh, SIN, x);
define_function!(Cosh, COS, x);
define_function!(Tanh, TAN, x);
define_function!(Coth, COT, x);
define_function!(Sech, SEC, x);
define_function!(Csch, CSC, x);

#[rustfmt::skip]
register_phantom_functions!(
    FRAC, PhantomFraction,
    ROOT, PhantomRoot,

    LOG | LG | LN, PhantomLog,

    SIN, PhantomSin,
    COS, PhantomCos,
    TAN, PhantomTan,
    COT, PhantomCot,
    SEC, PhantomSec,
    CSC, PhantomCsc,

    ARCSIN, PhantomArcsin,
    ARCCOS, PhantomArccos,
    ARCTAN, PhantomArctan,
    ARCCOT, PhantomArccot,
    ARCSEC, PhantomArcsec,
    ARCCSC, PhantomArccsc,

    SINH, PhantomSinh,
    COSH, PhantomCosh,
    TANH, PhantomTanh,
    COTH, PhantomCoth,
    SECH, PhantomSech,
    CSCH, PhantomCsch
);

#[rustfmt::skip]
register_phantom_operators!(
    ADD, PhantomAdd,
    SUBTRACT, PhantomSubtract,
    MULTIPLY, PhantomMultiply,
    DIVIDE, PhantomDivide,
    SUPER_SCRIPT, PhantomPower
);

macro_rules! define_math_enum {
    ($enum_ty: ident, $($ident: ident, $ty: ty),*) => {
        #[derive(Debug)]
        pub enum $enum_ty {
            $($ident($ty),)*
        }

        impl Function for $enum_ty {
            fn evaluate(&self) -> MathElement {
                match self {
                    $($enum_ty::$ident(elem) => elem.evaluate(),)*
                }
            }

            fn approximate(&self) -> DecimalScalar {
                match self {
                    $($enum_ty::$ident(elem) => elem.approximate(),)*
                }
            }
        }

        impl IntoRawExpr for $enum_ty {
            fn assemble(&self) -> String {
                match self {
                    $($enum_ty::$ident(elem) => elem.assemble(),)*
                }
            }
        }
    };
}

#[rustfmt::skip]
define_math_enum!(
    MathFunction,
    Add, Add,
    Subtract, Subtract,
    Multiply, Multiply,
    Divide, Divide,
    Power, Power,
    Fraction, Fraction,
    Root, Root,
    Log, Log,
    Sin, Sin,
    Cos, Cos,
    Tan, Tan,
    Cot, Cot,
    Sec, Sec,
    Csc, Csc,
    Arcsin, Arcsin,
    Arccos, Arccos,
    Arctan, Arctan,
    Arccot, Arccot,
    Arcsec, Arcsec,
    Arccsc, Arccsc,
    Sinh, Sinh,
    Cosh, Cosh,
    Tanh, Tanh,
    Coth, Coth,
    Sech, Sech,
    Csch, Csch
);
