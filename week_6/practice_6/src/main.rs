fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    //The to String converts the variable into a string object
    let n4 = n1 +&n2+&n3;
    /*n2 and n3 reference is passed
     only 3 variables can be catenated at mass */
    println!("\nThe {} is informed by the aspiration to train electrical/electronic engineering professionals in the areas of design, building and maintenance of electrical control systems",n4);
     // this is just to output the information which was stored in n4

    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1+&w2;
    //w2 reference is passed and the two string objects are stored in w3
    println!();
    println!("{} is aimed at developing competent ,creative,innovative,entrepreneural and ethically minded persons, that are capable of creating value in the diverse fields of Computer Science",w3);
//This just output the two Variables Stored in w3 and Writes some other Stuff after it
}
