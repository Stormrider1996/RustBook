// Introducing a type alias Thunk to reduce repetition
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

// The Never Type that Never Returns
fn bar() -> ! {
    // --snip--
}
