use crate::math::MathElement;

use super::{
    decl::{Prioritizable, Sin},
    Function,
};

impl Prioritizable for Sin {
    #[inline]
    fn priority(&self) -> u32 {
        10
    }
}

impl Function for Sin {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}
