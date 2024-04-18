//! All the possible symbols in LaTex.
//!
//! `&str`s are functions while `char`s are not.

pub const FUNC_BEGIN: char = '\\';
pub const WHITESPACE: char = ' ';

pub const PI: &str = "\\pi";
pub const E: &str = "e";

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
pub const LG: &str = "lg";
pub const LN: &str = "ln";

pub const SIN: &str = "sin";
pub const COS: &str = "cos";
pub const TAN: &str = "tan";
pub const COT: &str = "cot";
pub const SEC: &str = "sec";
pub const CSC: &str = "csc";

pub const ARCSIN: &str = "arcsin";
pub const ARCCOS: &str = "arccos";
pub const ARCTAN: &str = "arctan";
pub const ARCCOT: &str = "arccot";
pub const ARCSEC: &str = "arcsec";
pub const ARCCSC: &str = "arccsc";

pub const SINH: &str = "sinh";
pub const COSH: &str = "cosh";
pub const TANH: &str = "tanh";
pub const COTH: &str = "coth";
pub const SECH: &str = "sech";
pub const CSCH: &str = "csch";
