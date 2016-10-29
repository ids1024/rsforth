/// Represents a builtin function
#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Dot,
    Plus,
    Minus,
    Times,
    Divide,
}

impl Builtin {
    pub fn call(&self, stack: &mut Vec<i32>) {
        match *self {
            Builtin::Dot => print!("{}", stack.pop().unwrap()),
            Builtin::Plus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 + n2);
            }
            Builtin::Minus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 - n2);
            }
            Builtin::Times => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 * n2);
            }
            Builtin::Divide => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 / n2);
            }
        }
    }
}
