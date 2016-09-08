#[cfg(test)]
mod tests {
    use std::thread::JoinHandle;
    use std::thread::spawn;
    use std::sync::Arc;
    use lock_free_stack::Stack;

    #[test]
    fn test_size_returns_zero_size_for_empty_stack() {
        let stack: Stack<u32> = Stack::new();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_size_returns_size_one_for_stack_with_one_element() {
        let stack: Stack<u32> = Stack::new();
        stack.add(1);
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_size_returns_size_two_for_stack_with_two_elements() {
        let stack: Stack<u32> = Stack::new();
        stack.add(1);
        stack.add(2);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_add_returns_one_size_when_adding_to_an_empty_stack() {
        let stack: Stack<u32> = Stack::new();
        assert_eq!(stack.add(1), 1);
    }

    #[test]
    fn test_add_returns_size_two_when_adding_to_a_stack_with_one_element() {
        let stack: Stack<u32> = Stack::new();
        stack.add(1);
        assert_eq!(stack.add(2), 2);
    }

    #[test]
    fn test_size_returns_zero_size_after_removing_all() {
        let stack: Stack<u32> = Stack::new();
        stack.add(1);
        stack.remove_all();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_remove_all_returns_an_empty_iterator_for_empty_stack() {
        let stack: Stack<u32> = Stack::new();
        assert_eq!(stack.remove_all().next(), None);
    }

    #[test]
    fn test_remove_all_returns_a_one_item_iterator_for_stack_with_one_item() {
        let stack: Stack<u32> = Stack::new();
        stack.add(100);

        let mut iterator = stack.remove_all();
        assert_eq!(iterator.next(), Some(100));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_add_in_threads_remove_in_current() {
        let stack: Arc<Stack<u32>> = Arc::new(Stack::new());

        let mut threads: Vec<JoinHandle<()>> = vec!();
        for i in 0 .. 10 {
            let stack_ref = stack.clone();
            threads.push(spawn(move || {
                stack_ref.add(i);
            }));
        }

        for handle in threads {
            let _ = handle.join();
        }

        assert_eq!(stack.size(), 10);
        let mut values: Vec<u32> = stack.remove_all().collect();
        values.sort();

        assert_eq!(values, vec!(0,1,2,3,4,5,6,7,8,9));
    }
}
