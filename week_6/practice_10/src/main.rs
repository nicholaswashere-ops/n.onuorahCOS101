fn main() {
    let a = 20;
    let b = 30;
    //creating variables and assigning them values (Hardcoded)

    if (a> 10)&&(b>10){
        println!("True");
        //(&&)And statement prints true if the two conditions are true
    }
    let c =0;
    let d =30;

    //creating new variables and assigning them Values
    if (c>10)||(d>10){
        println!("True");
        //(||)OR statements prints true if any or both of the two conditins are true

    }
    let is_elder = false;
    //Automatically creates a bool datatype

    if !is_elder{
        println!("Not Elder");
        //(!)NOT Gives the oppsite of any data it is working with
    }
}
