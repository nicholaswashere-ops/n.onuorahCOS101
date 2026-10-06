fn main() {
    let arr:[i32;4]=[10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    for val in arr.iter(){//stores value in arr and repeats /iterates
        println!("value is : {}",val);//outputs val
    }

    
}
