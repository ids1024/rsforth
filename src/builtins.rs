#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    dot,
    plus,
    minus,
    times,
    divide,
}

impl Builtin {
    pub fn call(&self, stack: &mut Vec<i32>) {
        match *self {
            Builtin::dot => print!("{}", stack.pop().unwrap()),
            Builtin::plus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 + n2);
            }
            Builtin::minus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 - n2);
            }
            Builtin::times => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 * n2);
            }
            Builtin::divide => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 / n2);
            }
        }
    }
}
