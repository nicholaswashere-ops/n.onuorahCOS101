fn main() {
    let city_arr:[&str;5]= ["Abuja","Port-harcourt","Maidugiri","Kano","Lagos"];//creates a string literal array
    println!("array is {:?}",city_arr);
    println!("array size is :{}",city_arr.len());//outputs the length of the array

    for index in 0..5{//contains range of 0 to 4
        println!("City index {} is located in : {}",index,city_arr[index])
    }//repeats it from 0 to 5 with the five arrays

}
