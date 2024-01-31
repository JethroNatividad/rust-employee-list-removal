// Write a program that has a list of names, prompt a name to remove it from the list.
// Inputs: employee_name
// Process: search in list, remove
// Outputs: the updated employee list

fn remove_employee(employees: Vec<String>, remove_name: &str) -> Result<Vec<String>, &'static str> {
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
    use super::*;

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
            remove_employee(&employees, "John Smith"),
            vec![
                "Jackie Jackson",
                "Chris Jones",
                "Amanda Cullen",
                "Jeremy Goodwin"
            ]
        );
    }
}
fn main() {
    // Initialize employees

    // print, "There are 5 employees"
    // print employees

    // get remove employee name, "Enter an employee name to remove: "

    // remove employee
    println!("Hello, world!");
}
