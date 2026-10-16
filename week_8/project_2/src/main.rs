use::std::io;
fn main() {
    println!("Welcome to EY enterprise,Nigeria ");
    println!("You have currently been selected for interviewing and we would need you to input your credentials for allocating work");
    println!("For today, i will be your interviwer and my name is Mr.Rusty Richard, My Colleagues call me R`2");

let mut interviewee :Vec<(String,usize)> =Vec::new();
loop{


       let mut input1 = String::new();
       let mut input2 = String::new();
       let mut input3 = String::new();

        println!("Lets start with the credentials ");
        println!("Please input interviewee  name:");
       io::stdin().read_line(&mut input1).expect("I need a name to work with so something simple and with letters");
       let name: String = input1.trim().parse().expect("Your input isn't valid for interpretation");
        

      //For the user to type his name

      println!("Mr. or Mrs. {:?} can you input your number of years of experience in programming",name);
      println!("This will help in accessing your Qualification to work for EY");
      println!("We are also going to use it to access your position to work");
      println!("Number of years of experience:");
 
      // for the user to input his experience
      io::stdin().read_line(&mut input2).expect("I expect specific years and not float years. just approximate downwards and give me the integer years");
      let exp:usize = input2.trim().parse().expect("Please can you specifically put in an integer to work with for your interview");

        // Determine the experience and the name then store (Due to repetition)
        let _experience = checker(exp);
        println!("{:?} -> {:?}years of experience\n", name, exp);
        interviewee.push((name, exp));
        //we push the name and the years to the vector  outside the loop so that it can be accessed after the loop ondividually after repetitio

      println!("Do you still wish to continue? \n input ==(y/n)== in lowercase");
      io::stdin().read_line(&mut input3).expect("You are meant to just type (y/n)");
      let choice = input3.trim().to_lowercase();

      if choice== "y"{
        continue;

      }
      else {
        break;
      }
    }

  if interviewee.is_empty() {//this is if the interviewee is sad enough not to input anything
        println!("No interviewee's were entered.");// i made use of the opposite in anif and else statement because; it is easier to access false in general than determination of specific true 
    } 
    else {
        let mut grade = &interviewee[0];//introducing a mut statement with a beginning array and check with the fo statement below  
        for  candidate in & interviewee {
            if candidate.1 > grade.1 {//this is to check in the loop for what candidate matches with best
                grade = candidate;//To determine the particular interviwer whose experience is peculiar
            }
        }

                println!("SO far the Most Experienced is{:?} with  {:?}years Experienced ",grade.0,grade.1);
                /*This is to place the first array and the second array gotten from the initial candidate =interviewee
                calling the function to call the exp into it and give it a job*/
         }
    
}
// ths functions main work is to collect the integer experience of the user and put it under the range it falls under in the conditional statements and provide their jobs. 
fn checker(exp: usize) {
    if exp >=60{
        println!("I think you should consider retiring ,there isn't any work for you here let the young be employed\n Just go and be with your family ");
    }
    else if exp >=20{
        println!("Wonderful you qualified to be the\n (Chief of Technology)CTO ,Congratulations");
    }
    else if exp >= 10{
        println!("Wonderful you qualified to be the\n (Assistant Chief of Technology),Congratulations");

    }
   else if exp>= 7{   
        println!("Wonderful you qualified to be the\n (Senior Software engineer ),Congratulations");
    }
    else if exp>= 5{
            println!("Wonderful you qualified to be the\n (Junior Software engineer),Congratulations");
    }
    else if exp >= 3{
            println!("Wonderful you qualified to be the\n (Senior software analysts),Congratulations");
    }
    else if exp >= 2{
            println!("Wonderful you qualified to be the\n (Junior Software analysts),Congratulations");
    }
    else if exp >= 1{
    println!("Wonderful you qualified to work as  \n (Customer Service)");
    }
    else{
    println!("WELL I GUESS WERE DESPERATE.\n you qualified as an (Intern),Congratulations");
     }
     
}// each stayement is grouped to the range they fall under

