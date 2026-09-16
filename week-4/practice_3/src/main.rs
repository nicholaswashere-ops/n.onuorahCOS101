use std::io;
fn main() {

    let mut input1 = String::new ();
    let mut input2 = String::new();

    //User inputs base
    println!("Input base of the triangle ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    //User inputs height
    println!("Input height of the triangle");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let height:f32 = input2.trim().parse().expect("Not a valid number ");

    //calculated area if base is greater than 0
    
    if base > 0.0 {
        let area:f32 =(base * height)/2.0 ;
        println!("The Area of a triangle : {}",area);
}
}
