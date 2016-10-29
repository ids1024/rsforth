use stack::Stack;

/// Represents a builtin function
#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Dot,
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
}

impl Builtin {
    pub fn call(&self, stack: &mut Stack) {
        match *self {
            Builtin::Dot => print!("{}", stack.popi()),
            Builtin::Plus => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 + n2);
            }
            Builtin::Minus => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 - n2);
            }
            Builtin::Times => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 * n2);
            }
            Builtin::Divide => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 / n2);
            }
            Builtin::Equal => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushb(n1 == n2);
            }
            Builtin::LessThan => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushb(n1 < n2);
            }
            Builtin::GreaterThan => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushb(n1 > n2);
            }
            Builtin::LessEqual => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushb(n1 <= n2);
            }
            Builtin::GreaterEqual => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushb(n1 >= n2);
            }
        }
    }
}
