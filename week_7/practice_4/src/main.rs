use std::io;
fn add(a:i32,b:i32) {//calls two variable types in the function add
    let sum = a+b ;//adds the two a and b together

    println!("Sum of A and B = {}",sum);//output the sum of a and b
}

fn main(){
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 =input1.trim().parse().expect("Invalid input");//creates an input and store it in a

    let mut input2 = String::new();
    println!("Enter input for parameter B");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to input");//stores users input in b
    //calling the add  functions with arguments
    add(a,b);//the a and b are called again to specify what should run

}