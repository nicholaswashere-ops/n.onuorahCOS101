fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];
    //creates a vector for characters only  and uses ' punctuation as multiple chars but act individual

    let mut input1 = String::new();
    //creating an empty mutable String value 

    println!("Enter an index value between (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Please input between a range of 0..7");
    /*telling the user to input between a range of 8 that matches the values in v and since rust starts counting from 0 other than 1
    index is the non negative value which is smaller than the size of the vevctor 
    usize  is a datatype that assigns the values to a dynamic size but only works on usize and not isize e.t.c */
    let index:usize = input1.trim().parse().expect("Invalid input");

    //getting value at given index value 
    let ch: char =v[index];
    //creates the input as a char number when it falls under the digit array of the limited vector and matches them into an array


    println!("{} is the character for index [{}]\n",ch,index);

}
