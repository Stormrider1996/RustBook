Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.

//  A main function declaration containing one statement
fn main() {
    let y = 6;
}

// example of expressions 
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

{
    let x = 3;
    x + 1
}

//example of a function that returns a value
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

