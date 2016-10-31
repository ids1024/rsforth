use super::forthnum::ForthNum;

#[derive(Default)]
pub struct Stack {
    values: Vec<i32>,
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

    pub fn len(&self) -> i32 {
        self.values.len() as i32
    }
}
