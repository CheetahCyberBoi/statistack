#[cfg(test)]
mod tests {
    use statistack::stack::Stack;
    #[test]
    fn stack_push() {
        let mut stack: Stack<u32> = Stack::new(5, 0);
        stack.push(5).expect("Stack push operation failed!");
        assert_eq!(stack, Stack { stack: Box::new([5, 0, 0, 0, 0]), stack_pointer: 1, stack_length: 5 })
    }
    #[test]
    #[should_panic]
    fn stack_push_result() {
        let mut stack: Stack<u32> = Stack::new(1, 0);
        println!("{:?}", stack);
        stack.push(5).unwrap();
        stack.push(6).unwrap();
        
    }
    #[test]
    fn pop_test() {
        let mut stack: Stack<u32> = Stack::new(3, 0);
        stack.push(5).unwrap();
        stack.push(6).unwrap();
        println!("{:?}", stack);
        let val1 = stack.pop().unwrap();
        println!("{}", val1);
        let val2 = stack.pop().unwrap();
        println!("{}", val2);
        println!("{:?}", stack);

        assert_eq!(val1, 6);
        assert_eq!(val2, 5);

    }
}