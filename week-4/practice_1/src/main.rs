//introducing a standard input/output crate
use std::io;
fn main() {
    println!("\n Student Information Management System!");

    //input name
    println!("\nPlease Enter your name.");
    //introduce a variable name 
    let mut name =  String::new();
    // User inputs name
     io::stdin()
     .read_line (&mut name)
     .expect("Failed to read input");
    //output 
    println!("Your name is {} ", name);
    //input age 

    println!("\nEnter your age.");
    let mut age = String::new();
    //user inputs age
     io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 =age.trim().parse().expect("Input an integer");
    //output your age 
    println!("Your age is: {}", age); 
}
