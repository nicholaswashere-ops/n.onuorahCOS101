use std::io::Write;


fn main() {
    let  header = "Larger,Stout,Non-Alcoholic\n";
    let data_1 = "33Export,Legend,Maltina\n";
    let data_2 = "Desperados,TurboKing,AmstelMalta\n";
    let data_3 = "Gulder,,Fayrouz\n";
    let data_4 = "Heineken,,\n";
    let data_5 = "Star,,\n";

let mut copy = std::fs::File::create("Nigerian_Breweries.csv").expect("Create a tabular file");
   copy.write_all(header.as_bytes()).expect("There is an error");
   copy.write_all(data_1.as_bytes()).expect("UNwritten datatype");
   copy.write_all(data_2.as_bytes()).expect("UNwritten datatype");
   copy.write_all(data_3.as_bytes()).expect("UNwritten datatype");
   copy.write_all(data_4.as_bytes()).expect("UNwritten datatype");
   copy.write_all(data_5.as_bytes()).expect("UNwritten datatype");

   println!("Data Written to file");




    
}
