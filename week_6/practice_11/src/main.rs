fn main() {
    let a:i32=2; //The bit presentation is 10
    let b:i32=3; //The bit prresentation is 11

    let mut result:i32; //creating a mutable variable for continuous usage

    result= a&b; 
    println!("(a&b)=>{}",result);

    result=a|b;
    println!("(a|b)=>{}",result);

    result = a^b;
    println!("(a^b)=>{}",result);

    result = !b ;
    println!("(!b)=>{}",result);

    result = a<<b;
    println!("(a<<b)=>{}",result);

    result = a>>b;
    println!("(a>>b)=>{}",result);
    
}
