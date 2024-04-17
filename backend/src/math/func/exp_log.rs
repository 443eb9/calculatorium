use crate::math::MathElement;

use super::{
    decl::{Log, Power, Root},
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
