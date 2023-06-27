// Creating a new, empty String

let mut s = String::new();

// Using the to_string method to create a String from a string literal
let data = "initial contents";

let s = data.to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();

// Using the String::from function to create a String from a string literal
let s = String::from("initial contents");

// Appending a string slice to a String using the push_str method
let mut s = String::from("foo");
s.push_str("bar");

// Using a string slice after appending its contents to a String
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");

// Adding one character to a String value using push
let mut s = String::from("lo");
s.push('l');

// Using the + operator to combine two String values into a new String value
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this:
fn add(self, s: &str) -> String {}

// example of format macro 
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");

// using string slices to fetch pieces of a string 
let hello = "Здравствуйте";

let s = &hello[0..4]; // s would be "Зд" because cyrlic letters are 2 bytes

// chars function 
for c in "Зд".chars() {
    println!("{c}");
} // prints: 
//З
//д

// bytes function
for b in "Зд".bytes() {
    println!("{b}");
} // prints:
// 208
// 151
// 208
// 180