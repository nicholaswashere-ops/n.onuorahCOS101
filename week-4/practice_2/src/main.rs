use std::io;
fn main() {
    let mut input1 =String::new();
    let mut input2 =String::new();
    let mut input3 =String::new();
//user inputs 1st side of a triangle
    println!("Enter the first edge of the triangle:");
    io::stdin().read_line(&mut input1).expect("Invalid String ,Please insert a string");
    let a:f32 =input1.trim().parse().expect("Invalid number ,Please input a float");
//user inputs 2nd side of the triangle
    println!("Enter the second edge of the triangle");
    io::stdin().read_line(&mut input2).expect("Invalid String ,Please insert a string");
    let b:f32 =input2.trim().parse().expect("Invalid number ,Please input a float");
//user inputs 3rd side of the triangle
    println!("Enter the third edge of the triangle:");
    io::stdin().read_line(&mut input3).expect("Invalid String ,Please insert a string");
    let c:f32 =input3.trim().parse().expect("Invalid number ,Please input a float");
//CALCULATIONS
let s:f32 = (a+b+c)/2.0;
let mut area:f32 = s*(s-a)*(s-b)*(s-c);
area = area.sqrt();
println!("Area of a Triangle: {}", area);
}