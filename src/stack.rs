#[derive(Default)]
pub struct Stack {
    values: Vec<i32>,
}

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

impl Stack {
    pub fn push<T: ForthNum>(&mut self, value: T) {
        self.values.push(value.to_forth_num());
    }
    pub fn pop<T: ForthNum>(&mut self) -> T {
        if let Some(value) = self.values.pop() {
            T::from_forth_num(value)
        } else {
            panic!("Stack underflow");
        }
    }
    pub fn peak<T: ForthNum>(&self) -> T {
        if let Some(value) = self.values.last() {
            T::from_forth_num(*value)
        } else {
            panic!("Stack underflow");
        }
    }
}
