use std::io::Write;

fn main() {
    
    //Creating empty vectors for inserting values under their datatype

    let mut records =std::fs::File::create("PAU-SMIS.csv").expect("WAS unable to create file");
    //Creating a file for storing this in a tabular form
    let title = vec!["Students Name", "Matric Number", "Department","Level"];
    let student =  vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Bianca Edemoh"];
    let matric = vec!["ACC10211111","ECO10110101","CSC1038828","EEE11020202","MEE10202001"];//There is also another way of writing this by just inputing their first 3bits into the variable they are placed and when they appear and convert this into a numeric datatype
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level :Vec<usize>= vec![300,100,200,200,100];
//Creating of different vectors for accepting different daatatypes and there is no alphanumeric datatype so a string was used.
    let row1= format!("{},{},{},{}\n",title[0],title[1],title[2],title[3]);
    let row2= format!("{},{},{},{}\n",student[0],matric[0],department[0],level[0]);
    let row3= format!("{},{},{},{}\n",student[1],matric[1],department[1],level[1]);
    let row4= format!("{},{},{},{}\n",student[2],matric[2],department[2],level[2]);
    let row5= format!("{},{},{},{}\n",student[3],matric[3],department[3],level[3]);
//this is to convert the vectors into variables and the format is to sort them automatically and prevent repetition in writing also the\n is toskip a line for arranging when the program was run
//Reason:is that vectors don't run as_bytes but in another format with more syntax but this is shorter.another project might have that

    records.write_all(row1.as_bytes()).expect("Your title head couldn't compile");
    records.write_all(row2.as_bytes()).expect("Your names couldn't compile");
    records.write_all(row3.as_bytes()).expect("Your matric no. couldn't compile");
    records.write_all(row4.as_bytes()).expect("your department valus couldn't compile");
    records.write_all(row5.as_bytes()).expect("Your level couldn't compile");
    //The writing of the info into records and making use of bytes instead of as_bytes :they are vectors

    print!("PAU SIMS file has been created succesfully,Thank you!");
    //to output if the file is created.

    
}
