use std::io;

fn main() {
    // Display menu
    println!("P - Poundo Yam / Edikaikong Soup =>  ₦3,200");
    println!("F - Fried Rice & Chicken         =>  ₦3,000");
    println!("A - Amala & Ewedu Soup           =>  ₦2,500");
    println!("E - Eba & Egusi Soup             =>  ₦2,000");
    println!("W - White Rice & Stew            =>  ₦2,500");

    //  to read the food type
    println!("Enter food type (P/F/A/E/W):");
    println!("please type either P<F<E<A<W,in caps");

    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).unwrap();

    // to read the quantity
    println!("Enter the quantity of food by number portions:");
    println!("please type a number,THANK YOU");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).unwrap();
    

    let quantity: f64 = quantity.trim().parse().unwrap();
    //using float datatype

    //  to Determine price pair the food_type that the user inputed 
    let price = match food_type.trim().to_uppercase().as_str() {
        "P" => 3200.0,"F" => 3000.0,"A" => 2500.0,"E" => 2000.0,"W" => 2500.0,
        _ => {
           //if the user does not input a letter foodtype it will be outputed then jumped back into the running code
            println!("Invalid food type!,Please pick among the stored variable food types and type it as you see it, you can only have one meal ,Hungry man ");
            return;
        }
    };

    // Calculate total
    let total = price * quantity;

    // Calculate discount
    let discount = if total > 10_000.0 {
        total * 0.05
    } 
    else {
        0.0
    };

    let amount_to_pay = total - discount;

    // DISPLAYING RESULT IN TWO DECIMAL PLACES
    println!("Total: ₦{:.2}", total);
    println!("Discount: ₦{:.2}", discount);
    println!("Amount to pay: ₦{:.2}", amount_to_pay);
    println!("Cash transfers only");
    println!("Account Number:348-5679-6784");
    println!("Thank you for shopping come again");
}


