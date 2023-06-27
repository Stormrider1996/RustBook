// string slices
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// these are equal
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

// these are equal
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

// these are equal
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

// example 
// write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// array slice example
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);