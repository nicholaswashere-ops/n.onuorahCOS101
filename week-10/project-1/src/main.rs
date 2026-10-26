struct Customer{
     brand:String,
     quantity:usize,
     price:usize// made us of a usize due to the large size of the numbers
}
impl Customer{
    fn total(&self)->usize{
       3*self.quantity*self.price//this is to multiply the quantity with their price
    }//Since we are mmeant to find the
}
fn main() {
    let user1=Customer{
        brand:String::from("HP"),
        quantity:10,
        price:650_000
    };
    let user2=Customer{
        brand:String::from("IBM"),
        quantity:6,
        price:755_000
    };
    let user3=Customer{
        brand:String::from("Toshiba"),
        quantity:10,
        price:550_000
    };
    let user4=Customer{
        brand:String::from("Dell"),
        quantity:4,
        price:850_000
    };

    println!("Alaba ELectronics\n ,Owner:Mr. Ogbeifuna\n ");
    println!("The brand Sold :{} Laptop\n The Quantity Sold :{}pieces\n Each Price :N{}\n Total: N{}\n",user1.brand,user1.quantity,user1.price,user1.total());
    println!("The brand Sold :{} Laptop\n The Quantity Sold :{}pieces\n Each Price :N{}\n Total: N{}\n",user2.brand,user2.quantity,user2.price,user2.total());
    println!("The brand Sold :{} Laptop\n The Quantity Sold :{}pieces\n Each Price :N{}\n Total: N{}\n",user3.brand,user3.quantity,user3.price,user3.total());
    println!("The brand Sold :{} Laptop\n The Quantity Sold :{}pieces\n Each Price :N{}\n Total: N{}\n",user4.brand,user4.quantity,user4.price,user4.total());
//This is to output the result and the user_.total is to call the method above which works with the users details
}
/*From the question a customer buys 3 laptops of each brand (HP, IBM, Toshiba, Dell).  obtaining the  total cost. The quantities in stock are10, 6, 10, 4.
 it is then confirmed hat confirm that 3 of each is available. The prices are kept the same and not using the quantities and dividing them by their prices to obtain a single value and then *3 for each*/



 