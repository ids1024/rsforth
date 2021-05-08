use crate::builtins::Builtin;
use crate::state::InterpState;
use std::rc::Rc;

/// Represents a branch in the syntax tree
#[derive(Debug, PartialEq)]
pub enum Branch {
    Custom(Rc<Vec<Branch>>),
    Builtin(Builtin),
    Int(i32),
    Float(f32),
    IfElse(Vec<Branch>, Vec<Branch>),
    Dotquote(String),
}

impl Branch {
    pub fn call(&self, state: &mut InterpState) {
        match *self {
            Branch::Custom(ref branches) => {
                for branch in branches.iter() {
                    branch.call(state);
                }
            }
            Branch::Builtin(ref builtin) => builtin.call(state),
            Branch::Int(int) => state.stack.push(int),
            Branch::Float(float) => state.stack.push(float as i32), //XXX
            Branch::IfElse(ref ifbranches, ref elsebranches) => {
                if state.stack.pop() {
                    for branch in ifbranches.iter() {
                        branch.call(state);
                    }
                } else {
                    for branch in elsebranches.iter() {
                        branch.call(state);
                    }
                }
            }
            Branch::Dotquote(ref text) => print!("{}", text),
        }
    }
}
