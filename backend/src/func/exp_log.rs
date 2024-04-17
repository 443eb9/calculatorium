use crate::math::MathElement;

use super::{
    decl::{Lg, Ln, Log, Power, Root},
    Function, Operator,
};

impl Function for Root {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Operator for Power {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Function for Log {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Function for Lg {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}

impl Function for Ln {
    fn evaluate(&self) -> MathElement {
        todo!()
    }
}
