//NB the sruct is declared beforee the main function 
//Declaring a struct
struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    //initialize a structure
    let emp1 = Employee{
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:51
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sunday Pichai"),
        age:51
    };
    //pass emp1 and emp2 to dispay()
    display(emp1);
    display(emp2);
//this helps tom reduce repetiton
}
//fetch Values of Specific Structure fields using the operator and printit to the console
fn display(emp:Employee){
    println!("Name is :{}, Company is :{}, age is {}",emp.ceo,emp.company,emp.age);
}