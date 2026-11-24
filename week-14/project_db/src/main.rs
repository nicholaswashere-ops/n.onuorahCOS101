use std::io::Read;
use std::io;

fn main() {
    println!("Welcome to the Globacom Database \n ");
    let mut position = String::new();

    println!("input anyone of the following positions and be case sensitive \n(Administrator, Project manager, Employee, Customer, Vendor)");
    io::stdin()
        .read_line(&mut position)
        .expect("Rank not inputted successfully");

    let position = position.trim();

    if position == "Administrator" {
        let mut contents = String::new();
        let mut file1 = std::fs::File::open("globacom_dbase.sql").expect("sql wasn't created successfully");
        file1.read_to_string(&mut contents).expect("file not read successfully");
        print!("{}", contents);
    } else if position == "Project manager" {
        let mut contents2 = String::new();
        let mut file2 = std::fs::File::open("project_tb.sql").expect("sql wasn't successfully created");
        file2.read_to_string(&mut contents2).expect("File read unsuccessfully");
        print!("{}", contents2);
    } else if position == "Employee" {
        let mut contents3 = String::new();
        let mut file3 = std::fs::File::open("employee_tb.sql").expect("sql wasn't successfully created");
        file3.read_to_string(&mut contents3).expect("File read unsuccessfully");
        print!("{}", contents3);
    } else if position == "Customer" {
        let mut contents4 = String::new();
        let mut file4 = std::fs::File::open("customer_tb.sql").expect("sql wasn't successfully created");
        file4.read_to_string(&mut contents4).expect("File read unsuccessfully");
        print!("{}", contents4);
    } else if position == "Vendor" {
        let mut contents5 = String::new();
        let mut file5 = std::fs::File::open("dataplan_tb.sql").expect("sql wasn't successfully created");
        file5.read_to_string(&mut contents5).expect("File read unsuccessfully");
        print!("{}", contents5);
    } else {
        println!("no input matches the forms");
    }
}