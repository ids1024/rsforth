use std::rc::Rc;
use builtins::Builtin;
use stack::Stack;

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
    pub fn call(&self, stack: &mut Stack) {
        match self {
            &Branch::Custom(ref branches) => {
                for branch in branches.iter() {
                    branch.call(stack);
                }
            }
            &Branch::Builtin(ref builtin) => builtin.call(stack),
            &Branch::Int(int) => stack.pushi(int),
            &Branch::Float(float) => stack.pushi(float as i32), //XXX
            &Branch::IfElse(ref ifbranches, ref elsebranches) => {
                if stack.popb() {
                    for branch in ifbranches.iter() {
                        branch.call(stack);
                    }
                } else {
                    for branch in elsebranches.iter() {
                        branch.call(stack);
                    }
                }
            }
            &Branch::Dotquote(ref text) => print!("{}", text),
        }
    }
}
