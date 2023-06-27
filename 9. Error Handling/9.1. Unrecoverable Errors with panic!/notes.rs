// Attempting to access an element beyond the end of a vector, which will cause a call to panic!
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}

//
