// unsafe superpowers
Dereference a raw pointer
Call an unsafe function or method
Access or modify a mutable static variable
Implement an unsafe trait
Access fields of unions

// Creating raw pointers from references
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// Creating a raw pointer to an arbitrary memory address
let address = 0x012345usize;
let r = address as *const i32;

// Dereferencing raw pointers within an unsafe block
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

// Here is an unsafe function named dangerous that doesn’t do anything in its body:
unsafe fn dangerous() {}

unsafe {
    dangerous();
}

// Using the safe split_at_mut function
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);

//  Using unsafe code in the implementation of the split_at_mut function
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Declaring and calling an extern function defined in another language
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Defining and using an immutable static variable
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}

// Reading from or writing to a mutable static variable is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Defining and implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
