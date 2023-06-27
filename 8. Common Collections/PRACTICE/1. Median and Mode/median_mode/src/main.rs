use std::collections::HashMap;

fn find_median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    let n = numbers.len();

    if n % 2 == 0 {
        let middle_right = numbers[n / 2];
        let middle_left = numbers[n / 2 - 1];
        return (middle_left + middle_right) as f32 / 2.0;
    } else {
        return numbers[n / 2] as f32;
    }
}

fn find_mode(numbers: &Vec<i32>) -> Option<i32> {
    let mut frequency_map = HashMap::new();

    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut max_count = 0;

    for (&num, &count) in frequency_map.iter() {
        if count > max_count {
            max_count = count;
            mode = Some(num);
        }
    }

    mode
}

fn get_numbers() -> Vec<i32> {
    let mut numbers = String::new();
    std::io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    numbers
}

fn main() {
    println!("Enter numbers: ");
    let mut numbers = get_numbers();

    let median = find_median(&mut numbers);
    println!("Median: {}", median);

    if let Some(mode) = find_mode(&numbers) {
        println!("Mode: {}", mode);
    } else {
        println!("No mode found.");
    }
}
