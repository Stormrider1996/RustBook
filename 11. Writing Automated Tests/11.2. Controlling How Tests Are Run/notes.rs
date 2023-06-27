// We can specify part of a test name, and any test whose name matches that value will be run. For example, because two of our tests’ names contain add, we can run those two by running cargo test add:
// You can choose which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

$ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.62s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

// running 3 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok
// test tests::one_hundred ... ok

// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test one_hundred
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.69s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

// running 1 test
// test tests::one_hundred ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

$ cargo test add
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.61s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

// running 2 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

// Ignoring Some Tests Unless Specifically Requested
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

$ cargo test -- --ignored
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.61s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

// running 1 test
// test expensive_test ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

//    Doc-tests adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
