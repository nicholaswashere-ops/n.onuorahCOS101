use std::io;

fn main() {
    //mute the string inputs for storage
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    //output notice for the user to know what this does

    println!("Please input your  Quadratic equation Variables to calculate the discriminant:");
   
    //user inputs first variable
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let a:f64 = input1.trim().parse().expect("Not a valid number");
    
    //User inputs second input
    println!("Input your second input:");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let b:f64 = input2.trim().parse().expect("Not a valid number");
    
    //user inputs  third input
    println!("Input your third input :");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    //TO output the units in formulae
    println!("\nQuadratic Formula: {}x^2+{}x+{}",a,b,c);

    //discriminant
    let d = b * b - 4.0 * a * c;

    //To output the discriminant result
    println!("\nDiscriminant: {}",d);

    //A conditional  statement to respond to solutions greater than 0
    if d > 0.0{
        println!("Two Distinct Roots");
    }
    //A conditional statement to respond to solutions  equal 0
    else if d==0.0{
        println!("Exactly one real root");
    }
    //A conditional statement to respond to solutions less than 0
    else if d<0.0{
        println!("No real roots");
    }
    else {
        println!("Your inputs are meaningless")
    }

}