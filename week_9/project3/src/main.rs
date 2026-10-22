use std::io::Write;

fn main() {

    //Creating empty vectors for inserting values under their datatype

    let mut records =std::fs::File::create("The EFCC Merge.csv").expect("WAS unable to create file");
    //Creating a file for storing this in a tabular form
    let title = vec!["Commisioner", "Ministry", "Zone"];
    let table1=  vec!["Aigbogun Alamba Dauda","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let table2 = vec!["Internal Affairs","Justice","Defence","Power & Steel","Petroleum"];
    let table3 = vec!["South West","North East","South South","South West","South east"];
    //Writing vectors to Store the Available before arranged under a table.
// I placed them i values so that i can be able to sort it out as i work it out

    let heading =format!("{},{},{}\n",title[0],title[1],title[2]);
    let line1   =format!("{},{},{}\n",table1[0],table2[0],table3[0],);
    let line2   =format!("{},{},{}\n",table1[1],table2[1],table3[1]);
    let line3   =format!("{},{},{}\n",table1[2],table2[2],table3[2]);
    let line4   =format!("{},{},{}\n",table1[3],table2[3],table3[3]);
    let line5   =format!("{},{},{}\n",table1[4],table2[4],table3[4]);

    //After placing the values in their designated areas then write them into the lines according to how they will be printed

    records.write_all(heading.as_bytes()).expect("There is no heading to be displayed");
    records.write_all(line1.as_bytes()).expect("There is a fault on your first line");
    records.write_all(line2.as_bytes()).expect("There is a fault on your second line");
    records.write_all(line3.as_bytes()).expect("There is a fault on your third line");
    records.write_all(line4.as_bytes()).expect("There is a fault on your fourth line");
    records.write_all(line5.as_bytes()).expect("There is a fault on your fifth line");
//This is to copy the created values as they were arranged with the format style


println!("(The EFCC Merge)Your Retrieved files Have been merged succesfully.");





}
