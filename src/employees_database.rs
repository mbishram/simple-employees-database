use colored::Colorize;
use comfy_table::{Attribute, Cell, Table};
use std::collections::HashMap;

// ** Constants
pub const ORDER_ASC: &str = "asc";
pub const ORDER_DESC: &str = "desc";

// NOTE:
// It's better to use reference to a String (Vec<&string>) instead,
// If String are used, cloned() is needed when filter() is called.
// It is not efficient.
// i.e. delete() method.
#[derive(Debug)]
pub struct EmployeesDatabase(HashMap<String, Vec<String>>);

impl EmployeesDatabase {
    pub fn new() -> Self {
        EmployeesDatabase(HashMap::new())
    }

    pub fn create(&mut self, name: &str, department: &str) {
        let value = self.0.entry(department.to_string()).or_insert(vec![]);
        value.push(name.to_string());
    }

    pub fn delete(&mut self, name: &str, department: &str) {
        for (department_name, employees) in &mut self.0 {
            if (department != "" && department_name == department) || department == "" {
                *employees = employees
                    .iter()
                    .filter(|&employee| employee != &name.to_string())
                    .cloned()
                    .collect();
            }
        }
    }

    pub fn list_all_employees(&self, order: &str) {
        let mut table = Table::new();

        table.set_header([
            Cell::new("Employee").add_attribute(Attribute::Bold),
            Cell::new("Department").add_attribute(Attribute::Bold),
        ]);

        let mut data: Vec<[&String; 2]> = vec![];
        for (department, employees) in &self.0 {
            for employee in employees {
                data.push([employee, department])
            }
        }

        // TODO: Extract this function into an associated function sort_data();
        if order == ORDER_ASC || order == ORDER_DESC {
            data.sort_by(|a, b| {
                if order == ORDER_DESC {
                    b[0].cmp(a[0])
                } else {
                    a[0].cmp(b[0])
                }
            });
        }

        data.iter().for_each(|row_data| {
            table.add_row(row_data);
        });

        println!("{table}");
    }

    pub fn list_all_department(&self, order: &str) {
        let mut table = Table::new();

        table.set_header([Cell::new("Department").add_attribute(Attribute::Bold)]);

        let mut data: Vec<[&String; 1]> =
            self.0.iter().map(|(department, _)| [department]).collect();

        // TODO: Extract this function into an associated function sort_data();
        if order == ORDER_ASC || order == ORDER_DESC {
            data.sort_by(|a, b| {
                if order == ORDER_DESC {
                    b[0].cmp(a[0])
                } else {
                    a[0].cmp(b[0])
                }
            });
        }

        data.iter().for_each(|row_data| {
            table.add_row(row_data);
        });

        println!("{table}");
    }

    pub fn list_employees_on_department(&self, department: &str, order: &str) {
        let mut table = Table::new();

        table.set_header([Cell::new("Employee").add_attribute(Attribute::Bold)]);

        let mut data: Vec<[&String; 1]> = vec![];
        for (department_name, employees) in &self.0 {
            if department == department_name {
                for employee in employees {
                    data.push([employee])
                }
            }
        }

        // TODO: Extract this function into an associated function sort_data();
        if order == ORDER_ASC || order == ORDER_DESC {
            data.sort_by(|a, b| {
                if order == ORDER_DESC {
                    b[0].cmp(a[0])
                } else {
                    a[0].cmp(b[0])
                }
            });
        }

        data.iter().for_each(|row_data| {
            table.add_row(row_data);
        });

        println!("{table}");
    }

    pub fn help(&self) {
        println!("Usage:");
        println!(
            "  {}\t\t\tCreate a new {} on {}",
            "create EMPLOYEE DEPARTMENT".bold().blue(),
            "EMPLOYEE".bold().blue(),
            "DEPARTMENT".bold().blue()
        );
        println!(
            "  {}\t\t\t\tDelete an {}",
            "delete EMPLOYEE".bold().blue(),
            "EMPLOYEE".bold().blue()
        );
        println!(
            "  {}\t\t\tDelete an {} from {}",
            "delete EMPLOYEE DEPARTMENT".bold().blue(),
            "EMPLOYEE".bold().blue(),
            "DEPARTMENT".bold().blue()
        );
        println!(
            "  {}\t\t\tShow all employees with ascending ({}) or descending ({}) order",
            "list all employees [asc|desc]".bold().blue(),
            "asc".bold().blue(),
            "desc".bold().blue()
        );
        println!(
            "  {}\t\tShow all department with ascending ({}) or descending ({}) order",
            "list all department [asc|desc]".bold().blue(),
            "asc".bold().blue(),
            "desc".bold().blue()
        );
        println!(
            "  {}\t\tShow all employees on a given {} with ascending ({}) or descending ({}) order",
            "list employees DEPARTMENT [asc|desc]".bold().blue(),
            "DEPARTMENT".bold().blue(),
            "asc".bold().blue(),
            "desc".bold().blue()
        );
        println!("  {}\t\t\t\t\t\tShow help", "help".bold().blue());
        println!("  {}\t\t\t\t\t\tExit program", "exit".bold().blue());
    }
}
