// For example, the Iterator trait has an associated type named Item that stands for the type of the values it is iterating over1. The definition of the Iterator trait looks like this:
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Here, type Item is the associated type, and Self::Item is the placeholder type. When we implement the Iterator trait for a type, we need to specify what Item will be. For example:
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // some code
    }
}

// generic difference 
trait Add {
    type Output;
    fn add(&self, x: Self::Output, y: Self::Output) -> Self::Output;
}

impl Add for Calculator {
    type Output = i32;
    fn add(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

let calc = Calculator {};
let sum = calc.add(2, 3); // works fine
let product = calc.add(2.5, 3.5); // error: mismatched types
let concat = calc.add("Hello".to_string(), "World".to_string()); // error: mismatched types

// Default Generic Type Parameters
trait Add<T> {
    fn add(&self, other: T) -> Self;
}

trait Add<T> {
    fn add(&self, other: T) -> Self;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    fn add(&self, other: i32) -> Self {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add<f32> for Point {
    fn add(&self, other: f32) -> Self {
        Point {
            x: self.x + other as i32,
            y: self.y + other as i32,
        }
    }
}

// operator overloading
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2; // same as p1.add(p2)
println!("p3 = ({}, {})", p3.x, p3.y); // prints "p3 = (4, 6)"

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name in rust
<X as MyTrait>::hello(&x); // call hello method from MyTrait

MyTrait::hello(&x); // same as above, but Rust knows x is X

// supertraits 
// Define a trait named Animal that has a method named sound
trait Animal {
    fn sound(&self) -> &str;
}

// Define a trait named Dog that requires the Animal trait
// This means that any type that implements Dog must also implement Animal
trait Dog: Animal {
    fn breed(&self) -> &str;
}

// Define a struct named Labrador that represents a specific type of dog
struct Labrador {
    name: String,
}

// Implement the Animal trait for Labrador
impl Animal for Labrador {
    fn sound(&self) -> &str {
        "woof"
    }
}

// Implement the Dog trait for Labrador
// Notice that we can use the sound method from the Animal trait
impl Dog for Labrador {
    fn breed(&self) -> &str {
        "Labrador Retriever"
    }
}

// Create a function that takes a reference to a type that implements Dog
fn print_dog_info(dog: &dyn Dog) {
    println!("This dog is a {} and it says {}", dog.breed(), dog.sound());
}

fn main() {
    // Create an instance of Labrador
    let lab = Labrador {
        name: "Lucky".to_string(),
    };

    // Call the print_dog_info function with a reference to lab
    print_dog_info(&lab);
}

// this code prints 
This dog is a Labrador Retriever and it says woof

// Using the Newtype Pattern to Implement External Traits on External Types
struct MyNewType(ExternalType);

impl TraitName for MyNewType {
    // Implement trait methods here
}

impl TraitName for MyNewType {
    fn some_method(&self) {
        self.0.some_method(); // Delegate to the wrapped type
    }
}


