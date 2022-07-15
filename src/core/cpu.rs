pub struct Stack<T> {
    stack: Vec<T>
}
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new()
        }
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.stack.last_mut()
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

pub trait CPU {
    /// Fetch the next instruction from memory.
    fn fetch_opcode(self) -> u16;

    /// Decode and execute the instruction.
    fn execute_opcode(&mut self, opcode: u16);
}