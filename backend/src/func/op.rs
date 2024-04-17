use crate::math::MathElement;

use super::{
    decl::{Add, Divide, Multiply, Prioritizable, Subtract},
    Function,
};

impl Prioritizable for Add {
    #[inline]
    fn priority(&self) -> u32 {
        1
    }
}

impl Function for Add {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Prioritizable for Subtract {
    #[inline]
    fn priority(&self) -> u32 {
        1
    }
}

impl Function for Subtract {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Prioritizable for Multiply {
    #[inline]
    fn priority(&self) -> u32 {
        5
    }
}

impl Function for Multiply {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Prioritizable for Divide {
    #[inline]
    fn priority(&self) -> u32 {
        5
    }
}

impl Function for Divide {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}
