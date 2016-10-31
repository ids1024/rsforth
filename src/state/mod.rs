mod forthnum;
mod stack;
mod memory;

use self::stack::Stack;
use self::memory::Memory;

#[derive(Default)]
pub struct InterpState {
    pub stack: Stack,
    pub memory: Memory,
}
