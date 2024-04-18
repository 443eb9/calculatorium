use crate::{
    math::{
        func::{
            decl::{Add, Divide, Fraction, Multiply, Subtract},
            Function, Operator,
        },
        MathElement,
    },
    DecimalScalar,
};

impl Operator for Add {}

impl Function for Add {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.lhs().approximate() + self.rhs().approximate()
    }
}

impl Operator for Subtract {}

impl Function for Subtract {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.lhs().approximate() - self.rhs().approximate()
    }
}

impl Operator for Multiply {}

impl Function for Multiply {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.lhs().approximate() * self.rhs().approximate()
    }
}

impl Operator for Divide {}

impl Function for Divide {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.lhs().approximate() / self.rhs().approximate()
    }
}

impl Function for Fraction {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.num().approximate() / self.den().approximate()
    }
}
