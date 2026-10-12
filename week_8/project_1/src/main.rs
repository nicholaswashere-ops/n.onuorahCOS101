

use std::io;

fn check_level(profession: &str, experience: u32) -> String {
    let levels = vec![ "APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13","SES",];//to call a vector to hold the levels to input as the experience limit

    let office = vec![ "Intern","Administrator","Senior Administrator","Office Manager", "Director","CEO",];//to state a vector to hold the first set roles'

    let academic = vec!["--","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean",];//to state a vector to hold the second set roles'

    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner", ];//to state a vector to hold the third set roles'

    let teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal",];//to state a vector to hold the forth  set roles'

    let exp = if experience < 1 {//range less than 1 year of experience
        0
         } 
    else if experience < 5 {//range less than 5 year of experience
        1
    } 
    else if experience < 8 {//
        2
    } 
    else if experience < 10 {
        3
    } 
    else if experience < 13 {
        4
    } 
    else {
        5
    };

    let job = if profession == "Office Admin" {
        office[exp]
    } else if profession == "Academic" {
        academic[exp]
    } else if profession == "Lawyer" {
        lawyer[exp]
    } else if profession == "Teacher" {
        teacher[exp]
    } else {
        return String::from("Invalid profession!");
    };

    format!("{} - {}", levels[exp], job)
}

fn main() {// to output a table for the user to understand where they may fall under
   println!("=======The Automatic Work Assigner (LAGOS,Nigeria) FOR GRADUATES AND EXPERIENCED WORKERS=================");
   println!("=========================================================================================================");
   println!("publicservant      officeadmin           academic                 lawyer                    teacher    ");
   println!("APS 1-2               Intern                -                   Paralegal                    Placement   ");
   println!("APS 3-5               Administrstor      Research Assistant       Junior Associate     Classroom Teacher ");
   println!("APS 5-8             Senior Admiistrator   PhD candidate            Associate            Senior Teacher   ");
   println!("EL1 8-10             Office Manager       Post-Doc Researcher   Senior Associate 1-2    Leading Teacher  ");
   println!("EL2 10-13            Director             Senior lecturer       Senior Associate 3-4    Deputy Principal ");
   println!("SES                   CEO                    Dean                    Partner               Principal     ");
   println!("=========================================================================================================");

   println!("\nThis result may make mistakes so don't be oppressed if your weren't assigned ,it isn't from my end but your end ");
   println!("You can also file a report if the system was outrageous, to the contacts listed at the end");


    let mut name = String::new();//to call avariable to store the user's input on their name
    let mut profession = String::new();
    let mut input = String::new();

    println!("Enter staff name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter profession (Office Admin, Academic, Lawyer, Teacher):");
    io::stdin().read_line(&mut profession).expect("If your a public servant serve yourself and your community");

    println!("Enter years of experience:");
    io::stdin().read_line(&mut input).unwrap();

    let experience: u32 = input.trim().parse().unwrap_or(0);

    println!("\nStaff Name: {}", name.trim());
    println!("Staff Level: {}",//to output the staff's level in the print statement
        check_level(profession.trim(), experience)
    );
    println!("Contact 900-8906-789");
}

