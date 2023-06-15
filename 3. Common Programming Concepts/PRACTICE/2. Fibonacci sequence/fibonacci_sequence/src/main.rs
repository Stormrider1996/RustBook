fn main() {
    loop {
        println!("\nFind the nth Fibonacci number\n");
        let n = get_number_input();
        println!("{}", fibonacci_iterative(n));
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn get_number_input() -> u64 {
    loop {
        println!("Enter a number:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}
