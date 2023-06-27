use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command ('Add <name> to <department>', 'List <department>', 'List all', or 'Quit'):");
        let input = get_input();

        if input == "Quit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();

        match tokens[0] {
            "Add" => {
                if tokens.len() < 4 || tokens[2] != "to" {
                    println!(
                        "Invalid command. Please use the format 'Add <name> to <department>'."
                    );
                    continue;
                }

                let name = tokens[1].to_string();
                let department = tokens[3].to_string();

                company
                    .entry(department.clone())
                    .or_insert(vec![])
                    .push(name.clone());
                println!("Added {} to {} department.", name, department);
            }
            "List" => {
                if tokens.len() < 2 {
                    println!("Invalid command. Please specify a department or use 'List all'.");
                    continue;
                }

                if tokens[1] == "all" {
                    print_sorted_company(&company);
                } else {
                    let department = tokens[1].to_string();
                    print_sorted_department(&company, &department);
                }
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn print_sorted_department(company: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();

        println!("Employees in {} department:", department);
        for employee in sorted_employees {
            println!("{}", employee);
        }
    } else {
        println!("No employees found in {} department.", department);
    }
}

fn print_sorted_company(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort();

    for department in departments {
        print_sorted_department(company, department);
        println!();
    }
}
