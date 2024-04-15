//! All the possible symbols in LaTex.
//!
//! `&str`s are functions while `char`s are not.

pub const FUNC_BEGIN: char = '\\';

pub const PARENTHESES_L: char = '(';
pub const PARENTHESES_R: char = ')';
pub const SQUARE_BRACKET_L: char = '[';
pub const SQUARE_BRACKET_R: char = ']';
pub const CURLY_BRACKET_L: char = '{';
pub const CURLY_BRACKET_R: char = '}';

pub const ADD: &str = "+";
pub const SUBTRACT: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";

pub const SUPER_SCRIPT: &str = "^";
pub const SUB_SCRIPT: char = '_';

pub const FRAC: &str = "frac";
pub const ROOT: &str = "sqrt";

pub const LOG: &str = "log_";
pub const LG: &str = "lg_";
pub const LN: &str = "ln_";