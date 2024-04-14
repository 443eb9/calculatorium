use crate::latex::Position;

#[derive(Debug, Default)]
pub struct BracketStack {
    stack: Vec<Position>,
    depth: u32,
}

impl BracketStack {
    #[inline]
    pub fn push(&mut self, bracket: Position) {
        match bracket {
            Position::Left => self.depth += 1,
            Position::Right => self.depth -= 1,
        }
        self.stack.push(bracket);
    }

    #[inline]
    pub fn pop(&mut self) {
        if let Some(bracket) = self.stack.pop() {
            match bracket {
                Position::Left => self.depth -= 1,
                Position::Right => self.depth += 1,
            }
        }
    }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.depth
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = BracketStack::default();
        stack.push(Position::Left);
        assert_eq!(stack.depth(), 1);
        stack.push(Position::Left);
        assert_eq!(stack.depth(), 2);
        stack.push(Position::Right);
        assert_eq!(stack.depth(), 1);
        stack.push(Position::Left);
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
