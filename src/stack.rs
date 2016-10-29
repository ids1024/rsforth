#[derive(Default)]
pub struct Stack {
    values: Vec<i32>,
}

impl Stack {
    pub fn pushi(&mut self, value: i32) {
        self.values.push(value);
    }
    pub fn pushb(&mut self, value: bool) {
        let num = if value { -1 } else { 0 };
        self.values.push(num);
    }
    pub fn popi(&mut self) -> i32 {
        if let Some(value) = self.values.pop() {
            value
        } else {
            panic!("Stack underflow");
        }
    }
    pub fn popb(&mut self) -> bool {
        if let Some(value) = self.values.pop() {
            value != 0
        } else {
            panic!("Stack underflow");
        }
    }
}
