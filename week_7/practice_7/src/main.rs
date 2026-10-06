fn main() {
    //Array with data type (explicit integer datatype)
    let arr1:[i32;4] =[10,20,30,40];//it uses []brackets for datatype allocation and number of arrays
    println!("\nArray with data type");//skip line then output
    println!("array is {:?}",arr1);//{:?}is to output the arrays in the set
    println!("array size is :{}",arr1.len());//to output the length of bits in the array

    //Array without type (implicit float datatype), they ar allocated as floats automatically
    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];//it has no specified datatype
    println!("\nArray without datatype");
    println!("array is {:?}",arr2);//{:?}is to output the arrays in the set
    println!("array size is :{}",arr2.len());//to output the length of bits in the array

    /*Array with default values that creates and initializes 
    all its elements with a default value of -1.*/
    let arr3:[i32;8]=[-1;8];//the number of arrays assigned are more
    println!("\nArray with default values");
    println!("array is {:?}",arr3);//{:?}is to output the arrays in the set
    println!("array size is :{}",arr3.len());//to output the length of bits in the array
}