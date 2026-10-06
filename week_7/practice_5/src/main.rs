fn main() {
    let num:i32 = 5;
    mutate_num_zero(num);//calling the function mutate_num_zero And putting num's variable into it
    println!("The value of no is: {}",num);
}//to print num
//so therefore the value of the param_num is still 0 ,but the mutate... just copies it.

fn mutate_num_zero(mut param_num: i32){
    param_num = param_num*0;//param_num is saved as 0 
    println!("param_num value is {}",param_num);
}//creating an integer function
