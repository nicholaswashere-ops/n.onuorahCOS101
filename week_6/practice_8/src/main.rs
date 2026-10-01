fn main() {
    let num1 = 10;
    let num2 = 2;
    let mut result:i32;
    //with this result can be called multiple times with different variables
    //creating an unused variable
    result= num1 + num2;
    println!("Sum: {}",result);
    //outputs the value gotten from the addition of the two sets
    
    result =num1 -num2;
    println!("Difference: {}",result);
    //outputs the value gotten from the difference of the two variables

    result = num1*num2;
    println!("Product: {}",result);
    //outputs the value gotten from the product of the two sets

    result = num1/num2;
    println!("Quotient: {}",result);
    //outputs the value gotten from the quotient of the two sets

    result = num1%num2;
    println!("Remainder {}",result);
    // outputs the remainder gotten from the division of the two values
}
