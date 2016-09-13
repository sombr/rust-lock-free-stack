# Lock-Free (atomic-based) Stack
Rust Lock Free (Atomic) Collection that essentially is a Stack, but limited to a relatively slow "add" operation, but fast "remove_all" that removes everything from the collections and returns a reverse-order iterator.

## SYNTAX

Creating, adding elements, checking size:
```rust
let stack: Stack<u32> = Stack::new();
stack.add(50);
stack.add(10);
assert_eq!(stack.size(), 2);
```

Sharing between threads:
```rust
let stack: Arc<Stack<u32>> = Arc::new(Stack::new());

let mut threads: Vec<JoinHandle<()>> = vec!();
for i in 0 .. 10 {
    let stack_ref = stack.clone();
    threads.push(thread::spawn(move || {
        stack_ref.add(i);
    }));
}

for handle in threads {
    let _ = handle.join();
}

assert_eq!(stack.size(), 10);
```

Retrieving elements: this is an "unusual" implementation for Stack specifically focused on
fast batch processing, and thus only allowing to remove all the available elements from the stack (presumably in the worker-thread)
where they can be efficiently processed.
```rust
let stack: Arc<Stack<u32>> = Arc::new(Stack::new());

let stack_ref = stackc.clone();
let thread  = thread::spawn(move || {
    thread::sleep(time::Duration::from_millis(1000));
    for item in stack_ref.remove_all() {
        println!("item: {}", item);
    }
});

for i in 0 .. 10000 {
    stack.add(i);
}

thread.join();
```
