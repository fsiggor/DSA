pub struct MinStack {
    // Define your fields here
}

impl MinStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, val: i32) {
        todo!()
    }

    pub fn pop(&mut self) {
        todo!()
    }

    pub fn top(&self) -> i32 {
        todo!()
    }

    pub fn get_min(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn test_single_element() {
        let mut stack = MinStack::new();
        stack.push(42);
        assert_eq!(stack.top(), 42);
        assert_eq!(stack.get_min(), 42);
    }

    #[test]
    fn test_ascending_push() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.get_min(), 1);
        assert_eq!(stack.top(), 3);
    }

    #[test]
    fn test_descending_push() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(2);
        stack.push(1);
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 2);
        stack.pop();
        assert_eq!(stack.get_min(), 3);
    }

    #[test]
    fn test_duplicate_minimums() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(1);
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
    }

    #[test]
    fn test_negative_values() {
        let mut stack = MinStack::new();
        stack.push(-1);
        stack.push(-2);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn test_mixed_operations() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.get_min(), 3);
        assert_eq!(stack.top(), 7);
        stack.pop();
        assert_eq!(stack.top(), 3);
        assert_eq!(stack.get_min(), 3);
        stack.pop();
        assert_eq!(stack.get_min(), 5);
    }
}
