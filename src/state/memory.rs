use super::forthnum::ForthNum;

/// Provides space for memory to be allocated by Forth words like VARIABLE
/// and ALLOC
#[derive(Default)]
pub struct Memory {
    values: Vec<i32>,
}

impl Memory {
    pub fn get<T: ForthNum>(&self, addr: i32) -> T {
        if let Some(value) = self.values.get(addr as usize) {
            T::from_forth_num(*value)
        } else {
            panic!("Invalid memory address");
        }
    }
    pub fn set<T: ForthNum>(&mut self, addr: i32, value: T) {
        if addr as usize > self.values.len() - 1 {
            panic!("Invalid memory address");
        }
        self.values[addr as usize] = value.to_forth_num();
    }
    pub fn new<T: ForthNum>(&mut self, value: T) -> i32 {
        self.values.push(value.to_forth_num());
        self.here()
    }
    pub fn here(&self) -> i32 {
        self.values.len() as i32 - 1
    }
}
