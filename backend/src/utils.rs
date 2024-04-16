use crate::math::symbol::BracketState;

#[derive(Debug, Default)]
pub struct BracketStack {
    stack: Vec<BracketState>,
    depth: i32,
}

impl BracketStack {
    #[inline]
    pub fn push(&mut self, bracket: BracketState) {
        match bracket {
            BracketState::Open => self.depth += 1,
            BracketState::Close => self.depth -= 1,
        }
        self.stack.push(bracket);
    }

    #[inline]
    pub fn pop(&mut self) {
        if let Some(bracket) = self.stack.pop() {
            match bracket {
                BracketState::Open => self.depth -= 1,
                BracketState::Close => self.depth += 1,
            }
        }
    }

    #[inline]
    pub fn depth(&self) -> i32 {
        self.depth
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.depth >= 0
    }
}

#[macro_export]
macro_rules! sub_expr {
    ($exprs: expr, $nth: expr) => {
        std::mem::replace(&mut $exprs[$nth], LaTexExpression::default())
    };
}

#[macro_export]
macro_rules! take_elem_in_expr {
    ($expr: expr, $nth: expr) => {
        std::mem::replace(&mut $expr[$nth], LaTexElement::default())
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = BracketStack::default();
        stack.push(BracketState::Open);
        assert_eq!(stack.depth(), 1);
        stack.push(BracketState::Open);
        assert_eq!(stack.depth(), 2);
        stack.push(BracketState::Close);
        assert_eq!(stack.depth(), 1);
        stack.push(BracketState::Open);
        assert_eq!(stack.depth(), 2);
        stack.pop();
        assert_eq!(stack.depth(), 1);
        stack.pop();
        assert_eq!(stack.depth(), 2);
        stack.pop();
        assert_eq!(stack.depth(), 1);
        stack.pop();
        assert_eq!(stack.depth(), 0);
        stack.pop();
        assert_eq!(stack.depth(), 0);
        stack.pop();
        assert_eq!(stack.depth(), 0);
    }
}
