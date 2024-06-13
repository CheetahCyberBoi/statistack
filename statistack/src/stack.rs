
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Stack<T> {
    pub stack: Box<[T]>,
    pub stack_pointer: u64,
    pub stack_length: u64,
}

impl<T: Copy> Stack<T> {
    pub fn new(length: u64, initial_value: T) -> Stack<T> {
        //shame on me for a chatgpt solution
        let vec: Vec<T> = vec![initial_value; length as usize];
        Stack {
            stack: vec.into_boxed_slice(),
            stack_pointer: 0,
            stack_length: length,
        }
    }
    pub fn push(&mut self, value: T) -> Result<(), crate::error::Error> {
        if self.stack_pointer < self.stack_length {
            println!("pointer: {}", self.stack_pointer);

            self.stack[self.stack_pointer as usize] = value;
            self.stack_pointer += 1;

        } else {
            return Err(crate::error::Error::StackOverflowError);
        }
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.stack_pointer > 0 {
            println!("pointer: {}", self.stack_pointer);
            let value: T = self.stack[self.stack_pointer as usize];
            self.stack_pointer -= 1;
            return Some(value);
        } else {
            return None;
        }
    }
    pub fn len(&self) -> u64 {
        self.stack_length
    }
}