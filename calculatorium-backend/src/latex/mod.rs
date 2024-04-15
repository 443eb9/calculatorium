use crate::{
    expr::LaTexExpression,
    func::{decl::*, PhantomFunction},
    symbol::RealNumber,
};

pub mod symbols;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketState {
    Open,
    Close,
}

macro_rules! define_latex_elements {
    ($($fu_ident: ident, $fn_ty: ty),*) => {
        #[derive(Debug, Default)]
        pub enum LaTexElement {
            $($fu_ident($fn_ty),)*

            RealNumber(RealNumber),
            Parentheses(BracketState),
            Expression(LaTexExpression),
            PhantomFunction(Box<dyn PhantomFunction>),

            #[default]
            Dummy,
        }
    };
}

#[rustfmt::skip]
define_latex_elements!(
    Add, Add,
    Subtract, Subtract,
    Multiply, Multiply,
    Divide, Divide,
    Power, Power,

    Fraction, Fraction,
    Root, Root,
    Logarithm, Logarithm
);
