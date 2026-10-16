fn main() {

let v = vec!['R','U','S','T','A','C','I','A','N'];
//creates a character vector

let mut input1 = String::new();
println!("\n Enter an index between (0-8)");
std::io::stdin().read_line(&mut input1).expect("Failed to read input");
//calls a single line library then calls input , output docx.

//index is the non negative value which is smaller than the size of the vactor
let index:usize= input1.trim().parse().expect("Invalid input");

//getting value at given index value
let ch: Option<&char> =v.get(index);
//other than just converting the input into char .get places the m and matches them in an array to the variable vector v
value(ch)
//calling value function

}

fn value(n:Option<&char>)
{
    println!("Element of vector {:?}",n);
    //This function is a char function that just converts its input into n and runs it into it and keeps it there and continues from the function when called
}

