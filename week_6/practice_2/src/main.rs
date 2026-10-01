fn main() {
    let empty_string = String::new();
    println!("Length of empty_string is {}",empty_string.len());
    /*it creates a variable named empty_string and leaves it empty for multiple items as a String object
    .len counts the number of bytes
    */
    let content_string = String::from("ComputerScience");
    
    //it creates a string can still be edited as a String object .Although it is hardcoded
    println!("Length of content_string is {} ",content_string.len());

}
