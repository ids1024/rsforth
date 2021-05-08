mod forthnum;
mod memory;
mod stack;

use self::memory::Memory;
use self::stack::Stack;

#[derive(Default)]
pub struct InterpState {
    pub stack: Stack,
    pub memory: Memory,
}
