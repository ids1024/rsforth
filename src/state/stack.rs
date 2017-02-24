use super::forthnum::ForthNum;

/// Represents Forth's stack
#[derive(Default)]
pub struct Stack {
    values: Vec<ForthNum>,
}

impl Stack {
    pub fn push<T: Into<ForthNum>>(&mut self, value: T) {
        self.values.push(value.into());
    }

    pub fn pop<T: From<ForthNum>>(&mut self) -> T {
        if let Some(value) = self.values.pop() {
            value.into()
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn peak<T: From<ForthNum>>(&self) -> T {
        if let Some(value) = self.values.last() {
            (*value).into()
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn len(&self) -> i32 {
        self.values.len() as i32
    }
}
