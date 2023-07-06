//  Attempting to use a refutable pattern with let
let Some(x) = some_option_value;

// Using if let and a block with refutable patterns instead of let
if let Some(x) = some_option_value {
    println!("{}", x);
}
