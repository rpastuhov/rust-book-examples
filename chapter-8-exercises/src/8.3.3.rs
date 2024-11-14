
// Using a hash map and vectors, create a text interface to allow 
// a user to add employee names to a department in a company; for example, 
// “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve 
// a list of all people in a department or all people in the company by 
// department, sorted alphabetically.


use std::{collections::HashMap, io::stdin};



fn main() {
    println!("Enter a command (e.g., 'Add Sally to Engineering', 'List Engineering', 'List All'): ");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    departments.insert("engineering".to_string(), vec!["Sally".to_string()]);
    departments.entry("engineering".to_string()).or_insert(vec![]).push("Bob".to_string());
    departments.insert("sales".to_string(), vec!["Amir".to_string()]);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.to_lowercase();

        let sentence: Vec<&str> = input.trim().split_whitespace().collect();


        let name = match sentence.get(1) {
            Some(name) => name,
            None => ""
        };

        let department = match sentence.get(2) {
            Some(department) => department,
            None => ""
        };



        match sentence.get(0) {
            Some(&command) => {

                match command {
                    "add" => {
                        departments.entry(department.to_string())
                            .or_insert(vec![]).push(name.to_string());

                        println!("Added {} to {}", name, department);
                    },
                    "list" => println!("{:#?}", departments),
                    _ => println!("no command")
                }
            },
            None => {
                println!("Command error");
                continue;
            }
        }


    }
}