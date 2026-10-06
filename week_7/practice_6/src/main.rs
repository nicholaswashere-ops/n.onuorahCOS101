fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);//calling the function mutate_num_to_zero
    println!("The value of no is: {}",num);
}//to print num
//unlike value reference still retains the original value and doesn't copy it and uses & symbol with *__* as differences 
fn mutate_num_to_zero(param_num:&mut i32){
    *param_num = *param_num*0;
    println!("param_num value is {}",param_num);//de reference
}//creating an integer function
