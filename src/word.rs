use std::rc::Rc;
use builtins::Builtin;
use branch::Branch;

/// Represents a word implementation that can be bound to a name in the
/// Dictionary
#[derive(Clone, Debug, PartialEq)]
pub enum Word {
    Custom(Rc<Vec<Branch>>),
    Builtin(Builtin),
    Int(i32),
    Float(f32),
    Paren,
    Colon,
    Semicolon,
    If,
    Then,
    Else,
    Dotquote,
    Variable,
    Create,
}
