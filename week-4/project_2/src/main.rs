use std::io;
fn main() {
    
    //intoducing variable strorage to store the users input 
    let mut input1 =String::new();
    let mut input2 =String::new();
    let mut input3 =String::new();

    //TO inform the user the use of the program 
    println!("\nTHE INCENTIVE CALCULATOR :");
    println!("Input your employee name");

    //TO give THE EMPLOYEE A NAME
    io::stdin().read_line(&mut input1).expect("input a String");
    let name :String= input1.trim().parse().expect("Type a Name");

    //To inform the user that its a boolean input
    println!("Are you experienced or not?(true or false)");
    io::stdin().read_line(&mut input2).expect("Please state your Experience Status");
    let experience :bool = input2.trim().parse().expect("Please input a boolean (NO CAPS)");

    //To inform the user to input their age
    println!("Input your Age:");
    io::stdin().read_line(&mut input3).expect("Please input an integer");
    let age :i32=input3.trim().parse().expect("Please an integer");

    //These conditional statements are for determining the incentive according to the employees experience and age

    if experience== true && age > 40 {
        println!("{}, your annual incentive is N1,560,000",name);
    }
    else if experience== true && age >30 {
        println!("{}, your annual incentive is N1,480,000",name);
    }
    else if experience== true && age < 28 {
        println!("{}, your annual incentive is N1,300,000",name);
    }
    else if experience== false  {
        println!("{}, your annual incentive is N100,000 .This is due to your lack of experience",name);
    }
}
