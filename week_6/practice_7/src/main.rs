fn main() {
let k1 = "Yemisi".to_string();
let k2 = "Shyllon".to_string();
let k3 = "Museum".to_string();
let k4 = "of".to_string();
let k5 = "Art".to_string();
let k6 = "PAU".to_string();
/*this creates multiple variables with similar names which has nothing to do with their usage 
They are also stored as string objects*/
let k7 = format!("{} {} {} {} {} {} ",k1,k2,k3,k4,k5,k6);
//format macro Just organizes the stored variaables it is also another form of catenation
println!("\n {}",k7);
//to output the values stored in k7


}