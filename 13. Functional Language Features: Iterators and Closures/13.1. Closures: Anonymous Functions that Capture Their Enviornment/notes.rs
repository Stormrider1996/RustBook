// Adding optional type annotations of the parameter and return value types in the closure
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

// his illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
//The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to compile because the types will be inferred from their usage

// Attempting to call a closure whose types are inferred with two different types
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
// The compiler gives us this error:

// closuers can capture values from their environment in three ways, borrowing immutably, borrowing mutably, and taking ownership.
// Defining and calling a closure that captures an immutable reference
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// Defining and calling a closure that captures a mutable reference
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// Using move to force the closure for the thread to take ownership of list
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
// FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
// Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

