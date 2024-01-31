// Write a program that has a list of names, prompt a name to remove it from the list.
// Inputs: employee_name
// Process: search in list, remove
// Outputs: the updated employee list

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_employee() {
        let employees: Vec<&str> = vec![
            "John Smith",
            "Jackie Jackson",
            "Chris Jones",
            "Amanda Cullen",
            "Jeremy Goodwin",
        ];

        assert_eq!(
            remove_employee(employees, "John Smith"),
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
