fn main() {
    //Using Vec::new() , to create an empty
    let v : Vec<i64> = Vec::new();//of i64

    //printing the size of  the vector ,(But its empty??)
    println!("\nThe length of Vec::new is: {}",v.len());
    

    //using macro ,also providing it with 5 values which are represent singly and not together
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];


    println!("\n The length of vec macro is: {} ",v.len());
    //Vectors values can't be printed
}
