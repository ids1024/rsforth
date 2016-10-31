pub trait ForthNum {
    fn to_forth_num(&self) -> i32;
    fn from_forth_num(value: i32) -> Self;
}

impl ForthNum for i32 {
    fn to_forth_num(&self) -> i32 {
        *self
    }
    fn from_forth_num(value: i32) -> i32 {
        value
    }
}

impl ForthNum for bool {
    fn to_forth_num(&self) -> i32 {
        if *self { -1 } else { 0 }
    }
    fn from_forth_num(value: i32) -> bool {
        value != 0
    }
}

impl ForthNum for char {
    fn to_forth_num(&self) -> i32 {
        *self as i32
    }
    fn from_forth_num(value: i32) -> char {
        value as u8 as char
    }
}
