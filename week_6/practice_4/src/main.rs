fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";
    //  Assigning three variables With Strings
// creating a mutable String and storing it as a string object
    let mut school = "School of Science".to_string();
    //Appending the muted Variable School And Adding "and Technology"
    school.push_str(" and Technology");

    println!("My name is: {}",fullname);
    //to output the full name created above

    //to check the length
    println!("The length of my fullname is: {} ",fullname.len());
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",school);
}
