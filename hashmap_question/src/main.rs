use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    /*
     Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company
     for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
     a list of all people in a department or all people in the company by department, sorted alphabetically
    */

    // declare the variables (used strings and later converted to vectors to get input correctly)
    let mut name_string = String::new();
    let mut department_string = String::new();
    let mut department_and_names: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Welcome to the employee station please select a option");
        println!("1.Enter a new employee");
        println!("2.See the current department-employee status");
        println!("3.Quit");

        // getting selection from user
        let mut selection_string = String::new();
        io::stdin()
            .read_line(&mut selection_string)
            .expect("Failed to read line");

        let selection: i32 = selection_string
            .trim()
            .parse()
            .expect("Please type a valid integer");

        while selection == 1 {
            print!("Please enter the name of employee: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut name_string)
                .expect("Failed to read line");

            print!("Please enter the department that you want to add: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut department_string)
                .expect("Failed to read line");

            println!();

            // change strings to vectors
            let name = name_string.trim().to_string();
            let department = department_string.trim().to_string();

            // take the vectors you created and pass them into hash map
            department_and_names
                .entry(department)
                .or_insert_with(Vec::new)
                .push(name);

            // clear the strings after use to get clear output
            name_string.clear();
            department_string.clear();

            break;
        }
        while selection == 2 {
            println!();

            println!("Current department status:");

            // sort department
            let mut departments: Vec<_> = department_and_names.keys().collect();
            departments.sort();

            for department in departments {
                // sort employees
                let mut employees = department_and_names[department].clone();
                employees.sort();

                // print employees and departments
                println!("{:?}: {:?}", department, employees);
            }
            println!();
            break;
        }

        if selection == 3 {
            println!();
            println!("Quiting..");
            break;
        }
    }
}
