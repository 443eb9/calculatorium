use crate::{math::MathElement, DecimalScalar};

use super::{
    decl::{Cos, Sin, Tan},
    Function,
};

impl Function for Sin {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().sin()
    }
}

impl Function for Cos {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().cos()
    }
}

impl Function for Tan {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().tan()
    }
}
