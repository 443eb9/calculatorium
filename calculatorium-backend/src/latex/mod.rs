pub mod expr;
pub mod symbols;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketState {
    Open,
    Close,
}
