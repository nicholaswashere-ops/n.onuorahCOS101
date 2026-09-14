use std::io;
fn main() {
//mute two variables
    let mut input1 = String::new();
    let mut input2 = String::new();
//input a name
    println!("Enter Your name :");
    io::stdin().read_line(&mut input1).expect("Not a Valid String");
//input the name
    println!("Enter your age:");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let age:i32 = input2.trim().parse().expect("Not a valid number");
    
//create a conditional statement for the age and output the name if true or false
    if age >= 18{
        println!("Welcome to the party {}!",input1);

    }  else{
        println!("Oops, you are not of age to enter the party {}",input1);
    }
}   

