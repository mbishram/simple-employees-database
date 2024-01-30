// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use crate::employees_database::{EmployeesDatabase, ORDER_ASC, ORDER_DESC};
use colored::Colorize;
use std::io::{self, Write};

// ** Constants
const CREATE: &str = "create";
const DELETE: &str = "delete";
const LIST: &str = "list";
const HELP: &str = "help";
const EXIT: &str = "exit";

const KIND_ALL: &str = "all";
const KIND_EMPLOYEES: &str = "employees";

const SUB_KIND_EMPLOYEES: &str = "employees";
const SUB_KIND_DEPARTMENT: &str = "department";

mod employees_database;

fn main() {
    let mut database = EmployeesDatabase::new();

    let mut input = String::new();

    println!("Welcome to {} 👋", "Employees Database".bold().blue());
    println!(
        "Try some command. To view all available commands, type {}.",
        "\"help\"".bold()
    );

    loop {
        input.clear();

        print!("> ");
        io::stdout().flush().expect("Unable to flush stdout!");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line on stdin!");

        let input: Vec<&str> = input.trim().split_whitespace().collect();

        let command = input.get(0).unwrap_or(&"").to_lowercase();
        let command = command.as_str();
        match command {
            CREATE => {
                let name = input.get(1).unwrap_or(&"").trim();
                let department = input.get(2).unwrap_or(&"").trim();

                if input.len() > 3 {
                    println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
                    continue;
                }

                if name == "" {
                    println!("{}", "Employee name is missing!".bold().red());
                    continue;
                }

                if department == "" {
                    println!("{}", "Department is missing!".bold().red());
                    continue;
                }

                database.create(name, department);

                println!(
                    "{}{}{}{}",
                    name.bold().green(),
                    " added to ".green(),
                    department.bold().green(),
                    "!".green()
                );
            }
            DELETE => {
                let name = input.get(1).unwrap_or(&"").trim();
                let department = input.get(2).unwrap_or(&"").trim();

                if input.len() > 3 {
                    println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
                    continue;
                }

                if name == "" {
                    println!("{}", "Employee name is missing!".bold().red());
                    continue;
                }

                database.delete(name, department);

                if department == "" {
                    println!(
                        "{}{}{}",
                        "All employees named ".green(),
                        name.bold().green(),
                        " has been removed!".green(),
                    );
                } else {
                    println!(
                        "{}{}{}{}{}",
                        "All employees named ".green(),
                        name.bold().green(),
                        " has been removed from ".green(),
                        department.bold().green(),
                        "!".green()
                    );
                }
            }
            LIST => {
                let kind = input.get(1).unwrap_or(&"").trim();
                let sub_kind = input.get(2).unwrap_or(&"").trim();
                let order = input.get(3).unwrap_or(&ORDER_ASC).trim();

                if input.len() > 4 {
                    println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
                    continue;
                }

                if !(order == ORDER_ASC || order == ORDER_DESC) {
                    println!("{}", "Order must be asc or desc".bold().red());
                    continue;
                }

                match kind {
                    KIND_ALL => match sub_kind {
                        SUB_KIND_EMPLOYEES => {
                            database.list_all_employees(order);
                        }
                        SUB_KIND_DEPARTMENT => {
                            database.list_all_department(order);
                        }
                        _ => {
                            println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
                            continue;
                        }
                    },
                    KIND_EMPLOYEES => {
                        if sub_kind == "" {
                            println!("{}", "Department is missing!".bold().red());
                            continue;
                        }

                        database.list_employees_on_department(sub_kind, order);
                    }
                    _ => {
                        println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
                        continue;
                    }
                }
            }
            HELP => {
                database.help();
            }
            EXIT => {
                println!("Good bye 😊");
                break;
            }
            _ => {
                println!("Invalid command 🥺. Try {}.", "\"help\"".bold());
            }
        }
    }
}
