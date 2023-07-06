// Using the dereference operator to follow a reference to an i32 value
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
} // x = 5, y = 5

// Using the dereference operator on a Box<i32>
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining MyBox<T> type
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// deref coercions
// A hello function that has the parameter name of type &str
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// The code we would have to write if Rust didn’t have deref coercion
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

// Notes
// 1. Implementing the Deref trait allows customization of the behavior of the dereference operator (*).
// 2. Regular references can be followed to access the value they point to using the dereference operator.
// 3. Box<T> can be used like a reference, and the dereference operator works the same way on Box<T> as on regular references.
// 4. The author defines a custom smart pointer type called MyBox<T> and encounters an error when trying to use the dereference operator on it.
// 5. The Deref trait is implemented for MyBox<T> to enable dereferencing with the * operator.
// 6. Deref coercion is a feature that automatically converts references to types implementing Deref trait into references to the types they point to.
// 7. Deref coercion allows functions and methods to accept different types of references without explicitly adding references or dereferences.
// 8. Deref coercion can be used with custom smart pointer types like MyBox<T> to automatically convert them to the types they contain when passed as function arguments.
