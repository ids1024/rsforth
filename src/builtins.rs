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
            ( | $($aname:ident : $atype:ty),+ | $value:expr ) => {
                {
                    $(
                        let $aname: $atype = stack.pop();
                    )+
                    stack.push($value)
                }
            }
        }

        use self::Builtin::*;
        match *self {
            Dot => print!("{}", stack.pop::<i32>()),
            Plus => stackexpr!(|n2: i32, n1: i32| n1 + n2),
            Minus => stackexpr!(|n2: i32, n1: i32| n1 - n2),
            Star => stackexpr!(|n2: i32, n1: i32| n1 * n2),
            Slash => stackexpr!(|n2: i32, n1: i32| n1 / n2),
            Abs => stackexpr!(|n: i32| n.abs()),
            And => stackexpr!(|n2: i32, n1: i32| n1 & n2),
            Or => stackexpr!(|n2: i32, n1: i32| n1 | n2),
            Xor => stackexpr!(|n2: i32, n1: i32| n1 ^ n2),
            Equals => stackexpr!(|n2: i32, n1: i32| n1 == n2),
            LessThan => stackexpr!(|n2: i32, n1: i32| n1 < n2),
            GreaterThan => stackexpr!(|n2: i32, n1: i32| n1 > n2),
            LessEqual => stackexpr!(|n2: i32, n1: i32| n1 <= n2),
            GreaterEqual => stackexpr!(|n2: i32, n1: i32| n1 >= n2),
            Emit => print!("{}", stack.pop::<char>()),
            Dup => {
                let n: i32 = stack.peak();
                stack.push(n);
            }
            Swap => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n2);
                stack.push(n1);
            }
            Over => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1);
                stack.push(n2);
                stack.push(n1);
            }
            Rot => {
                let n3: i32 = stack.pop();
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n1);
                stack.push(n3);
                stack.push(n2);
            }
            Tuck => {
                let n2: i32 = stack.pop();
                let n1: i32 = stack.pop();
                stack.push(n2);
                stack.push(n1);
                stack.push(n2);
            }
            Drop => {
                stack.pop::<i32>();
            }
        }
    }
}
