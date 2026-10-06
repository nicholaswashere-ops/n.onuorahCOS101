fn main() {
    let name = "Aisha Lawal";
    //Automatically assigns it a string variable
    let uni:&str ="Pan-Atlantic University";
    //Saved as a String slice/literal
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    //it outputs the name, uni and Address
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);



//it replaces the variables with And freezes them
    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    //it outputs the department and School
    println!("Department: {}, \nSchool: {}",department,school);
println!("{} {} {} {} {} ",name, uni,addr,department,school);

}
