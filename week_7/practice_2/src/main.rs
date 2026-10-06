use std::io;

fn checker(){//calling a function named checker
    let mut input = String::new();//saving an empty variable
    io::stdin().read_line(&mut input).expect("Failed to read input");//reads users input
    let ch:char = input.trim().parse().expect("Invalid input");//cinverts it to character

    if ch >= '0' && ch <= '9'{
        println!("Character '{}' is a digit",ch);//runs if the character is in the range of 0 to 9

    }
    else{
        println!("Character '{}' is not a digit",ch);//runs if the char doesn't fall under the implication of the if statement
         }

}
fn main() {
    println!("Welcome! This program checks whether a character variable contains a digit or not. \n please input a character");
    //calls the function:checker into the main function
     checker()
}
