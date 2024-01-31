// Write a program that has a list of names, prompt a name to remove it from the list.
// Inputs: employee_name
// Process: search in list, remove
// Outputs: the updated employee list

fn remove_employee(
    employees: Vec<String>,
    remove_name: String,
) -> Result<Vec<String>, &'static str> {
    if employees.contains(&remove_name) {
        Ok(employees
            .iter()
            .filter(|&x| x != &remove_name)
            .cloned()
            .collect())
    } else {
        Err("String not found in vector")
    }
}

#[cfg(test)]
mod tests {
    use super::remove_employee;

    #[test]
    fn test_remove_employee() {
        let employees: Vec<String> = vec![
            "John Smith".to_string(),
            "Jackie Jackson".to_string(),
            "Chris Jones".to_string(),
            "Amanda Cullen".to_string(),
            "Jeremy Goodwin".to_string(),
        ];

        assert_eq!(
            remove_employee(employees.clone(), "John Smith".to_string()),
            Ok(vec![
                "Jackie Jackson".to_string(),
                "Chris Jones".to_string(),
                "Amanda Cullen".to_string(),
                "Jeremy Goodwin".to_string()
            ])
        );

        assert_eq!(
            remove_employee(employees, "Nonexistent Employee".to_string()),
            Err("String not found in vector")
        );
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // Initialize employees
    let employees: Vec<String> = vec![
        "John Smith".to_string(),
        "Jackie Jackson".to_string(),
        "Chris Jones".to_string(),
        "Amanda Cullen".to_string(),
        "Jeremy Goodwin".to_string(),
    ];

    // print, "There are 5 employees"
    println!("There are {} employees", employees.len());
    // print employees
    for employee in employees {
        println!("{}", employee);
    }

    // get remove employee name, "Enter an employee name to remove: "

    // remove employee
    println!("Hello, world!");
}
