pub struct MyQueue {
    stk1: Vec<i32>,
    stk2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            stk1: Vec::new(),
            stk2: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stk1.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.stk2.is_empty() {
            while let Some(val) = self.stk1.pop() {
                self.stk2.push(val);
            }
        }
        self.stk2.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        if self.stk2.is_empty() {
            self.stk1.first().unwrap().abs()
        } else {
            self.stk2.last().unwrap().abs()
        }
    }

    pub fn empty(&self) -> bool {
        self.stk1.is_empty() && self.stk2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);
    }

    #[test]
    fn test_empty_queue() {
        let queue = MyQueue::new();
        assert_eq!(queue.empty(), true);
    }

    #[test]
    fn test_push_and_pop_single() {
        let mut queue = MyQueue::new();
        queue.push(5);
        assert_eq!(queue.pop(), 5);
        assert_eq!(queue.empty(), true);
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
    }

    #[test]
    fn test_interleaved_push_pop() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.pop(), 1);
        queue.push(3);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let mut queue = MyQueue::new();
        queue.push(7);
        assert_eq!(queue.peek(), 7);
        assert_eq!(queue.peek(), 7);
        assert_eq!(queue.empty(), false);
    }

    #[test]
    fn test_multiple_operations() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        queue.push(3);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
        assert_eq!(queue.empty(), true);
    }
}
