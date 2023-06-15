fn main() {
    loop {
        println!(
            "\nPress 1 to convert celsius to fahrenheit\nPress 2 to convert fahrenheit to celsius"
        );

        let choice = get_user_choice();

        match choice {
            1 => {
                let temperature = get_temperature();
                convert_celsius_to_fahrenheit(temperature);
            }
            2 => {
                let temperature = get_temperature();
                convert_fahrenheit_to_celsius(temperature);
            }
            _ => {
                println!("\nInvalid choice!\n");
                continue;
            }
        }
    }
}

fn get_user_choice() -> u32 {
    loop {
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("\nPlease type a number\n"),
        };
    }
}

fn get_temperature() -> f64 {
    loop {
        println!("\nEnter the temperature");

        let mut temp = String::new();
        std::io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("\nPlease type a number\n"),
        };
    }
}

fn convert_celsius_to_fahrenheit(celsius: f64) {
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("\nThe temperature in Fahrenheit is {}", fahrenheit);
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("\nThe temperature in Celsius is {}", celsius);
}
