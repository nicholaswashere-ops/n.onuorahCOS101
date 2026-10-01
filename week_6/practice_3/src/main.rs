fn main() {
    let name1 = "Ayomide Adesokan";
    //to output name1
    println!("My name is {}",name1);
    //Rust assigns variable name1 with a string slice

     //to find and replace the given value with an Assigned value
     let name2 = name1.replace("Ayomide","Adebare");
     //to output the replaced name2 variable
    println!("You can also call me {}",name2);
     //Rust assigns a variable faculty to a string slce automatica

    let faculty ="Faculty of Science and Technology";
     //to find and replace the given value with an Assigned value
    let school = faculty.replace("Faculty","School");
     //to output the replaced variable
    println!("I am a student of the {}",school)
}
