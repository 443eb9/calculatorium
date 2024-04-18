use crate::{
    math::{
        func::{
            decl::{
                Arccos, Arccot, Arccsc, Arcsec, Arcsin, Arctan, Cos, Cosh, Cot, Coth, Csc, Csch,
                Sec, Sech, Sin, Sinh, Tan, Tanh,
            },
            Function,
        },
        MathElement,
    },
    DecimalScalar,
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

impl Function for Cot {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().tan()
    }
}

impl Function for Sec {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().sin()
    }
}

impl Function for Csc {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().cos()
    }
}

impl Function for Arcsin {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().asin()
    }
}

impl Function for Arccos {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().acos()
    }
}

impl Function for Arctan {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().asin()
    }
}

impl Function for Arccot {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        (1. / self.x().approximate()).atan()
    }
}

impl Function for Arcsec {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        (1. / self.x().approximate()).asin()
    }
}

impl Function for Arccsc {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        (1. / self.x().approximate()).acos()
    }
}

impl Function for Sinh {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().sinh()
    }
}

impl Function for Cosh {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().cosh()
    }
}

impl Function for Tanh {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.x().approximate().tanh()
    }
}

impl Function for Coth {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().tanh()
    }
}

impl Function for Sech {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().cosh()
    }
}

impl Function for Csch {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        1. / self.x().approximate().sinh()
    }
}
