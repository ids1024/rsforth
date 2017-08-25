/// Represents a value in Forth's stack
#[derive(Clone, Copy)]
pub struct ForthNum {
    value: i32,
}

impl From<i32> for ForthNum {
    fn from(value: i32) -> ForthNum {
        ForthNum { value: value }
    }
}

impl From<ForthNum> for i32 {
    fn from(num: ForthNum) -> i32 {
        num.value
    }
}

impl From<bool> for ForthNum {
    fn from(value: bool) -> ForthNum {
        ForthNum {
            value: if value {
                -1
            } else {
                0
            },
        }
    }
}

impl From<ForthNum> for bool {
    fn from(num: ForthNum) -> bool {
        num.value != 0
    }
}

impl From<char> for ForthNum {
    fn from(value: char) -> ForthNum {
        ForthNum { value: value as i32 }
    }
}

impl From<ForthNum> for char {
    fn from(num: ForthNum) -> char {
        num.value as u8 as char
    }
}
