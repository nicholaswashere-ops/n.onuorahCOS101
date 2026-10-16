fn main() {
    //Create an empty vector called city
    let mut city : Vec<String> = Vec::new();
    //print City Vector
    println!("The City vector has element {}",city.len());//which is empty


    //Push new elements into
    let mut input1 = String::new();
    println!("How many cities do you want to enter ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    //stores the input into city_num as an i32 value
    for count in 0..city_num{//converts city_num value to the limit
    let mut input2 = String::new();
    println!("Enter City {}",count+1);//it adds 1 to the count
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let new_city:String = input2.trim().parse().expect("Invalid input");
    city.push(new_city);
    //sends the new city to city new 

    }
println!("Your preferred cities are:\n");
let mut count =1;
//loop to iterate elements in the vector
for i in city{
    //iterating through i on the vector
    println!("{} {}",count,i);
    count+=1
}


}
