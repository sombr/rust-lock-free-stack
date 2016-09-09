mod lock_free_stack;

use std::env;
use lock_free_stack::Stack;

struct GGG {
    val: usize
}

fn main() {
    let stack: Stack<Box<GGG>> = Stack::new();
    let to: usize = env::args().nth(1).unwrap().parse().unwrap();
    for i in 0..to {
        stack.add(Box::new(GGG { val: i }));
    }

    println!("{} ", stack.size());
}
