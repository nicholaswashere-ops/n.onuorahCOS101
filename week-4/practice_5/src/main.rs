  use std::io;
  fn main() {
    let mut input =String::new();

    println!("\n Enter Your height (in centimeters)");
    io::stdin().read_line(&mut input).expect("Not a valid String");
    let height : f32 = input.trim().parse().expect("Not a valid Number");
    //giving a conditional statement to produce an output with the range of 150.0 to 170.0
    if height >= 150.0 && height <= 170.0{
        println!("You are of average height");
    }
    /*using another conditional statement to obtain a range of 170.0 to 195.0
     with the same height used in the first condition if proved false in the first statement*/
     else if height > 170.0 &&height <= 195.0
     {
        println!("You are tall");
     }   
//using the same inputed height if the two statements proved false, output this
     else if height < 150.0 && height > 100.0
     {
        println!("You are a dwarf");
     } 
     else
     {
        println!("Abnormal height");
     }












}
