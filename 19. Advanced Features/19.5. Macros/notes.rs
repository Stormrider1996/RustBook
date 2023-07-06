// A simplified version of the vec! macro definition
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
} // ( $( $x:expr ),* ) means variable that has nested variables spaced with a , and it can be empty or not with * 

// custom derived macros 
// This is the crate where we define our macro
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// This attribute tells the compiler to use this function as a custom derive macro
#[proc_macro_derive(SetFields)]
pub fn set_fields_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let name = input.ident;

    // Generate the code for the set_fields method
    let expanded = quote! {
        impl #name {
            pub fn set_fields(&mut self, id: i64, value: Option<String>) {
                self.id = id;
                self.value = value;
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

// This is the crate where we use our macro
// We need to import the macro crate and use its attribute
extern crate set_fields_derive;
use set_fields_derive::SetFields;

// We can now use our custom derive macro on any struct that has id and value fields
#[derive(SetFields)]
struct Example {
    id: i64,
    value: Option<String>,
}

fn main() {
    // We can now call the set_fields method on our struct
    let mut example = Example {
        id: 42,
        value: Some("Hello".to_string()),
    };
    println!("{:?}", example); // Example { id: 42, value: Some("Hello") }
    example.set_fields(99, None);
    println!("{:?}", example); // Example { id: 99, value: None }
}

// attribute macros 
// This crate must have the proc-macro crate type
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// Define the attribute macro
#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function item
    let input = parse_macro_input!(item as ItemFn);

    // Get the function name and body
    let name = &input.sig.ident;
    let body = &input.block;

    // Generate a new function with a println! statement
    let output = quote! {
        fn #name() {
            println!("Calling {}", stringify!(#name));
            #body
        }
    };

    // Return the generated function as a token stream
    output.into()
}

// Use the attribute macro on a function
#[log]
fn foo() {
    println!("Hello, world!");
}

fn main() {
    foo();
}

// functional macros
# [proc_macro]
pub fn add (input: TokenStream) -> TokenStream {
  // parse the input as two expressions separated by a comma
  // return the output as an expression that adds them
}

let sum = add! (2, 3);