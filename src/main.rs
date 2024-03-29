use std::io;
use std::io::Write;

// Write a program that has a list of names, prompt a name to remove it from the list.
// Inputs: employee_name
// Process: search in list, remove
// Outputs: the updated employee list

fn remove_employee(
    employees: Vec<String>,
    remove_name: &String,
) -> Result<Vec<String>, &'static str> {
    if employees.contains(&remove_name) {
        Ok(employees
            .iter()
            .filter(|&x| x != remove_name)
            .cloned()
            .collect())
    } else {
        Err("Employee not found.")
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
            remove_employee(employees.clone(), &"John Smith".to_string()),
            Ok(vec![
                "Jackie Jackson".to_string(),
                "Chris Jones".to_string(),
                "Amanda Cullen".to_string(),
                "Jeremy Goodwin".to_string()
            ])
        );

        assert_eq!(
            remove_employee(employees, &"Nonexistent Employee".to_string()),
            Err("Employee not found.")
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
    let mut employees: Vec<String> = vec![
        "John Smith".to_string(),
        "Jackie Jackson".to_string(),
        "Chris Jones".to_string(),
        "Amanda Cullen".to_string(),
        "Jeremy Goodwin".to_string(),
    ];

    loop {
        // print, "There are n employees"
        println!("There are {} employees", employees.len());
        // print employees
        for employee in &employees {
            println!("{}", employee);
        }

        loop {
            // get remove employee name, "Enter an employee name to remove: "
            let remove_name: String = get_input("Enter an employee name to remove: ");
            // remove employee
            match remove_employee(employees.clone(), &remove_name) {
                Ok(updated_employees) => {
                    println!("'{}' removed.", &remove_name);
                    employees = updated_employees;
                    break;
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
