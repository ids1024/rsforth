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
        macro_rules! stackexpr {
            ( $($aname:ident : $atype:ty),+ => $value:expr ) => {
                {
                    $(
                        let $aname: $atype = stack.pop();
                    )+
                    stack.push($value)
                }
            }
        }
        match *self {
            Builtin::Dot => print!("{}", stack.pop::<i32>()),
            Builtin::Plus => stackexpr!(n2: i32, n1: i32 => n1 + n2),
            Builtin::Minus => stackexpr!(n2: i32, n1: i32 => n1 - n2),
            Builtin::Star => stackexpr!(n2: i32, n1: i32 => n1 * n2),
            Builtin::Slash => stackexpr!(n2: i32, n1: i32 => n1 / n2),
            Builtin::Abs => stackexpr!(n: i32 => n.abs()),
            Builtin::And => stackexpr!(n2: i32, n1: i32 => n1 & n2),
            Builtin::Or => stackexpr!(n2: i32, n1: i32 => n1 | n2),
            Builtin::Xor => stackexpr!(n2: i32, n1: i32 => n1 ^ n2),
            Builtin::Equals => stackexpr!(n2: i32, n1: i32 => n1 == n2),
            Builtin::LessThan => stackexpr!(n2: i32, n1: i32 => n1 < n2),
            Builtin::GreaterThan => stackexpr!(n2: i32, n1: i32 => n1 > n2),
            Builtin::LessEqual => stackexpr!(n2: i32, n1: i32 => n1 <= n2),
            Builtin::GreaterEqual => stackexpr!(n2: i32, n1: i32 => n1 >= n2),
            Builtin::Emit => print!("{}", stack.pop::<char>()),
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
