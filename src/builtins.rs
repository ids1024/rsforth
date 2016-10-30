use stack::Stack;

/// Represents a builtin function
#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Abs,
    And,
    Or,
    Xor,
    Equals,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Emit,
    Dup,
    Swap,
    Over,
    Rot,
    Tuck,
    Drop,
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
            Builtin::Star => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 * n2);
            }
            Builtin::Slash => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 / n2);
            }
            Builtin::Abs => {
                let n = stack.popi();
                stack.pushi(n.abs());
            }
            Builtin::And => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 & n2);
            }
            Builtin::Or => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 | n2);
            }
            Builtin::Xor => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1 ^ n2);
            }
            Builtin::Equals => {
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
            Builtin::Emit => {
                let c = stack.popc();
                print!("{}", c);
            }
            Builtin::Dup => {
                let n = stack.peaki();
                stack.pushi(n);
            }
            Builtin::Swap => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n2);
                stack.pushi(n1);
            }
            Builtin::Over => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1);
                stack.pushi(n2);
                stack.pushi(n1);
            }
            Builtin::Rot => {
                let n3 = stack.popi();
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n1);
                stack.pushi(n3);
                stack.pushi(n2);
            }
            Builtin::Tuck => {
                let n2 = stack.popi();
                let n1 = stack.popi();
                stack.pushi(n2);
                stack.pushi(n1);
                stack.pushi(n2);
            }
            Builtin::Drop => {
                stack.popi();
            }
        }
    }
}
