/// Queue implementation utilizing 2 stacks (for learning purposes).
///
/// * The first stack is used for insertion:
///     * When a value is inserted it goes into the in stack
///
/// * The second stack is used for popping:
///     * When a pop is requested, it commes out of the out stack.
///     * If this stack is empty, then all values of the in stack
///     are popped and inserted into the out stack (which reverses
///     the stack order; ensuring they will come out in the original
///     order they went to the in stack).
pub struct Queue<T> {
    in_stack: Vec<T>,
    out_stack: Vec<T>,
}

#[allow(dead_code)]
impl<T> Queue<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Queue {
            in_stack: Vec::default(),
            out_stack: Vec::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.in_stack.len() + self.out_stack.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.out_stack.is_empty() {
            self.out_stack = self.in_stack.iter().rev().map(|x| *x).collect();
            self.in_stack = Vec::default();
        }

        self.out_stack.pop()
    }

    pub fn insert(&mut self, value: T) {
        self.in_stack.push(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque as Deque;

    #[test]
    fn default_constructor() {
        let q = Queue::<i32>::new();

        assert_eq!(q.len(), 0);
    }

    #[test]
    fn pop_empty_queue() {
        let mut q = Queue::<f64>::new();
        assert_eq!(q.pop(), None);

        // compare with the std implementation for correctness
        let mut q = Deque::<f64>::new();
        assert_eq!(q.pop_front(), None);
    }

    #[test]
    fn insert_empty_queue() {
        let mut q = Queue::<f64>::new();
        q.insert(7.0);
        assert_eq!(q.len(), 1);

        // compare with the std implementation for correctness
        let mut q = Deque::<f64>::new();
        q.push_back(7.0);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn pop() {
        let mut q = Queue::<f64>::new();
        q.insert(7.0);
        q.insert(10.0);

        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), Some(7.0));
        assert_eq!(q.len(), 1);
        assert_eq!(q.pop(), Some(10.0));
        assert_eq!(q.pop(), None);
        assert_eq!(q.len(), 0);

        // compare with the std implementation for correctness
        let mut q = Deque::<f64>::new();
        q.push_back(7.0);
        q.push_back(10.0);

        assert_eq!(q.len(), 2);
        assert_eq!(q.pop_front(), Some(7.0));
        assert_eq!(q.len(), 1);
        assert_eq!(q.pop_front(), Some(10.0));
        assert_eq!(q.pop_front(), None);
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn pop_then_insert_then_pop() {
        let mut q = Queue::<&str>::new();
        q.insert("12");
        q.insert("23");
        q.insert("34");

        assert_eq!(q.len(), 3);
        assert_eq!(q.pop(), Some("12"));
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), Some("23"));
        assert_eq!(q.len(), 1);

        q.insert("45");
        assert_eq!(q.len(), 2);
        q.insert("56");
        assert_eq!(q.len(), 3);
        assert_eq!(q.pop(), Some("34"));

        // compare with the std implementation for correctness
        let mut q = Deque::<&str>::new();
        q.push_back("12");
        q.push_back("23");
        q.push_back("34");

        assert_eq!(q.len(), 3);
        assert_eq!(q.pop_front(), Some("12"));
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop_front(), Some("23"));
        assert_eq!(q.len(), 1);

        q.push_back("45");
        assert_eq!(q.len(), 2);
        q.push_back("56");
        assert_eq!(q.len(), 3);
        assert_eq!(q.pop_front(), Some("34"));
    }
}
