pub struct MyQueue {
    // Define your fields here
}

impl MyQueue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, x: i32) {
        todo!()
    }

    pub fn pop(&mut self) -> i32 {
        todo!()
    }

    pub fn peek(&self) -> i32 {
        todo!()
    }

    pub fn empty(&self) -> bool {
        todo!()
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
