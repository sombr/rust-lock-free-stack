use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

#[allow(dead_code)]
struct Node<K> {
    size: usize,
    next: *mut Node<K>,
    payload: K
}

#[allow(dead_code)]
pub struct StackTailIterator<M> {
    top: *mut Node<M>
}

#[allow(dead_code)]
pub struct Stack<T> {
    top: AtomicPtr<Node<T>>
}

impl<T> Stack<T> {
    #[allow(dead_code)]
    pub fn new() -> Stack<T> {
        Stack { top: AtomicPtr::new(ptr::null_mut()) }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        let top_ptr = self.top.load(Ordering::Relaxed);
        if top_ptr.is_null() {
            0
        } else {
            unsafe { (&*top_ptr).size }
        }
    }

    #[allow(dead_code)]
    pub fn add(&self, value: T) -> usize {
        let mut top_ptr = self.top.load(Ordering::Relaxed);

        let new_top = Box::into_raw(Box::new(
            Node { size: 0, next: ptr::null_mut(), payload: value }
        ));

        loop {
            let new_size = if top_ptr.is_null() { 1 } else { unsafe { (&*top_ptr).size + 1 } };
            unsafe {
                (&mut *new_top).size = new_size;
                (&mut *new_top).next = top_ptr;
            };

            let previous_top = self.top.compare_and_swap(
                top_ptr,
                new_top,
                Ordering::Relaxed
            );

            if previous_top == top_ptr {
                return new_size;
            } else {
                top_ptr = previous_top;
            }
        }
    }

    #[allow(dead_code)]
    pub fn remove_all(&self) -> StackTailIterator<T> {
        let mut top_ptr = self.top.load(Ordering::Relaxed);

        loop {
            let previous_top = self.top.compare_and_swap(
                top_ptr,
                ptr::null_mut(),
                Ordering::Relaxed
            );

            if previous_top == top_ptr {
                return StackTailIterator { top: top_ptr }
            } else {
                top_ptr = previous_top;
            }
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let _ = self.remove_all();
    }
}

impl<M> Iterator for StackTailIterator<M> {
    type Item = M;

    fn next(&mut self) -> Option<M> {
        if self.top.is_null() {
            None
        } else {
            let boxed_node = unsafe { Box::from_raw(self.top) };
            self.top = boxed_node.next;

            Some(boxed_node.payload)
        }
    }
}

impl<M> Drop for StackTailIterator<M> {
    fn drop(&mut self) {
        for _ in self {}
    }
}
