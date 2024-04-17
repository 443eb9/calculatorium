use crate::math::MathElement;

use super::{
    decl::{Add, Divide, Fraction, Multiply, Subtract},
    Function, Operator,
};

impl Operator for Add {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Operator for Subtract {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Operator for Multiply {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Operator for Divide {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Function for Fraction {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}
