use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct GlobalStack<T> {
    inner: Arc<Mutex<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut stack = self.inner.lock().unwrap();
        stack.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut stack = self.inner.lock().unwrap();
        stack.pop()
    }

    pub fn peek(&self) -> Option<T> 
    where
        T: Clone,
    {
        let stack = self.inner.lock().unwrap();
        stack.last().cloned()
    }
}

#[cfg(test)]
mod thread_tests {
    use super::GlobalStack;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_thread_safety() {
        let stack = Arc::new(GlobalStack::new());
        let mut handles = vec![];

        // Spawn multiple threads that push to the stack
        for i in 0..10 {
            let stack_clone = Arc::clone(&stack);
            let handle = thread::spawn(move || {
                stack_clone.push(i);
            });
            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        // Check the stack length
        let stack_contents = stack.inner.lock().unwrap();
        assert_eq!(stack_contents.len(), 10);
    }

    #[test]
    fn test_shared_mutation_threads() {
        let stack = Arc::new(GlobalStack::new());
        let stack_clone1 = Arc::clone(&stack);
        let stack_clone2 = Arc::clone(&stack);

        let handle1 = thread::spawn(move || {
            stack_clone1.push(1);
            stack_clone1.push(2);
        });

        let handle2 = thread::spawn(move || {
            stack_clone2.push(3);
            stack_clone2.push(4);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Popping elements in any order since we can't guarantee thread execution order
        let mut results = vec![];
        for _ in 0..4 {
            results.push(stack.pop());
        }

        results.sort();
        assert_eq!(results, vec![Some(1), Some(2), Some(3), Some(4)]);
    }
}
