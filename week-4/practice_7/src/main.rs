use std::io;

fn main() {
// user inputs integer
    println!("Enter a number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut num:i32 = input1.trim().parse().expect("Faeiled to read input");
/* a range limit is declared in the code
to express a loop for numbers from input to 10 others are not acceptable*/
    while num < 10 {
        println!("Inside loop number value is {}",num);
        num+=1
    } 
    println!("outside loop number value is {}", num);
}
