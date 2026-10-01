fn main() {
    let  A :i32 = 10;
    let  B :i32 = 20;
    //creating two immutable values 

    println!("Value of A:{}",A);
    println!("Value of B:{}",B);
    //to output the data stored in other variables
 
    //if bool answers were required its just to make res a bool datatype
    let mut res = A>B;
    println!("A greater than B:{}",res);
    //creates a mutable variable and outputs its value

    res = A<B;
    println!("A lesser than B:{}",res);
    //changing the variable stored in res and outputs its new input

    res = A>=B;
    println!("A is greater than or equal to B:{}",res);
    //changing the variable stored in res and outputs its new input
    
    res = A<=B;
    println!("A is lesser than or equal to B:{}",res);
    //changing the variable stored in res and outputs its new input
    
    res = A==B;
    println!("A is equal to B:{}",res);
    //changing the variable stored in res and outputs its new input
    
    res = A!=B;
    println!("A is not equal to B: {}",res);
    //changing the variable stored in res and outputs its new input
   
}
