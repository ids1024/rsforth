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
            Builtin::Dot => print!("{}", stack.pop::<i32>()),
            Builtin::Plus => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 + n2);
            }
            Builtin::Minus => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 - n2);
            }
            Builtin::Star => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 * n2);
            }
            Builtin::Slash => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 / n2);
            }
            Builtin::Abs => {
                let n: i32 = stack.pop();
                stack.push(n.abs());
            }
            Builtin::And => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 & n2);
            }
            Builtin::Or => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 | n2);
            }
            Builtin::Xor => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 ^ n2);
            }
            Builtin::Equals => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 == n2);
            }
            Builtin::LessThan => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 < n2);
            }
            Builtin::GreaterThan => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 > n2);
            }
            Builtin::LessEqual => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 <= n2);
            }
            Builtin::GreaterEqual => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1 >= n2);
            }
            Builtin::Emit => {
                let c: char = stack.pop();
                print!("{}", c);
            }
            Builtin::Dup => {
                let n: i32 = stack.peak();
                stack.push(n);
            }
            Builtin::Swap => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n2);
                stack.push(n1);
            }
            Builtin::Over => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1);
                stack.push(n2);
                stack.push(n1);
            }
            Builtin::Rot => {
                let n3: i32 = stack.pop();
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1);
                stack.push(n3);
                stack.push(n2);
            }
            Builtin::Tuck => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n2);
                stack.push(n1);
                stack.push(n2);
            }
            Builtin::Drop => {
                stack.pop::<i32>();
            }
        }
    }
}
