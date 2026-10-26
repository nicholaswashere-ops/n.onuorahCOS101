struct Employee{
    name:String,
    company:String,
    age:u32
}//Struct-Structure
fn main() {
    let emp1 =Employee{
        company:String::from("Ernst & Young"),
        name:String::from("Ebibiong Jessica"),
        age:25
    };
    //NB that Structcts use , to skip a line or enter another variable because they don't close  yet 
    println!("Name = {} \n",emp1.name);
    println!("Company = {}\n",emp1.company);
    println!("Age ={}",emp1.age);
}
