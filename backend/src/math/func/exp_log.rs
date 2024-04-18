use crate::{
    math::{
        func::{
            decl::{Log, Power, Root},
            Function, Operator,
        },
        MathElement,
    },
    DecimalScalar,
};

impl Function for Root {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.deg().approximate().powf(1. / self.rad().approximate())
    }
}

impl Operator for Power {}

impl Function for Power {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.base().approximate().powf(self.exp().approximate())
    }
}

impl Function for Log {
    fn evaluate(&self) -> MathElement {
        todo!()
    }

    #[inline]
    fn approximate(&self) -> DecimalScalar {
        self.anti().approximate().log(self.base().approximate())
    }
}
