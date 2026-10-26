fn add_one(e: &mut i32){
    *e+=1;
    //this reference equation only works for the copy and adds 1
}

fn main() {
    let mut i = 3;
    add_one(&mut i);//sends i to the function
    println!("{}",i);

}
