use state::InterpState;

/// Represents a builtin function
#[derive(Debug, Clone, PartialEq, Variants)]
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
    LShift,
    RShift,
    Equals,
    NotEquals,
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
    Fetch,
    Store,
    Here,
    Comma,
    Depth,
    Allot,
}

impl Builtin {
    pub fn word(&self) -> &'static str {
        match *self {
            Builtin::Dot => ".",
            Builtin::Plus => "+",
            Builtin::Minus => "-",
            Builtin::Star => "*",
            Builtin::Slash => "/",
            Builtin::Abs => "abs",
            Builtin::And => "and",
            Builtin::Or => "or",
            Builtin::Xor => "xor",
            Builtin::LShift => "lshift",
            Builtin::RShift => "rshift",
            Builtin::Equals => "=",
            Builtin::NotEquals => "<>",
            Builtin::LessThan => "<",
            Builtin::GreaterThan => ">",
            Builtin::LessEqual => "<=",
            Builtin::GreaterEqual => ">=",
            Builtin::Emit => "emit",
            Builtin::Dup => "dup",
            Builtin::Swap => "swap",
            Builtin::Over => "over",
            Builtin::Rot => "rot",
            Builtin::Tuck => "tuck",
            Builtin::Drop => "drop",
            Builtin::Here => "here",
            Builtin::Fetch => "@",
            Builtin::Store => "!",
            Builtin::Comma => ",",
            Builtin::Depth => "depth",
            Builtin::Allot => "allot",
        }
    }

    pub fn call(&self, state: &mut InterpState) {
        let stack = &mut state.stack;

        /// Allows defining a builtin using closure-like syntax. Note that the
        /// arguments are in reverse order, as that is how the stack is laid out.
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
            LShift => stackexpr!(|u: i32, n: i32| n << u),
            RShift => stackexpr!(|u: i32, n: i32| n >> u),
            Equals => stackexpr!(|n2: i32, n1: i32| n1 == n2),
            NotEquals => stackexpr!(|n2: i32, n1: i32| n1 != n2),
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
                stack.push(n2);
                stack.push(n3);
                stack.push(n1);
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
            Fetch => stackexpr!(|addr: i32| state.memory.get::<i32>(addr)),
            Store => {
                let addr: i32 = stack.pop();
                let n: i32 = stack.pop();
                state.memory.set(addr, n);
            }
            Here => stack.push(state.memory.here()),
            Comma => {
                state.memory.new(stack.pop::<i32>());
            }
            Depth => {
                let len = stack.len();
                stack.push(len);
            }
            Allot => {
                let n = stack.pop();
                if n < 0 {
                    // TODO Free memory
                } else {
                    for _ in 0..n {
                        state.memory.new(0);
                    }
                }
            }
        }
    }
}
