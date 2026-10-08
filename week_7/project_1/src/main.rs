use std::io;
fn main() {
    //output the table for the user to know the available shapes and formulas
    println!("======================================================================");
    println!("----------------SHAPE CALCULATOR--------------------------------------");
    println!("Pick the particular shape you want to calculate and input their values");
    println!("trapezium-----Area= height/2 * (base1 +base2)");
    println!("rhombus-------Area= 1/2 * diagonal1 * diagonal2");
    println!("parralelogram-Area= base * altitude");
    println!("cube----------Surface Area= 6 * side * side");
    println!("cylinder------Volume= pie *radius * radius * height ");//pie will be hard coded 
    println!("======================================================================");
    //for the user to input the desired shape to solve
    let mut z =String::new();
    io::stdin().read_line(&mut z).expect("please input a valid shape, first letter is without caps");
    let shape:String= z.trim().parse().expect("Please all in snake case(lower letters)only");
   //to save the users input as a string


    if shape == "trapezium"{//to call the function trapezium if the user types trapezium
        trapezium();
    } 

    else if shape == "rhombus" {//to call the function rhombus if the user types rhombus
        rhombus();
    }
    else if shape == "parralelogram"{//to call the function parralelogram if the user inputs parralelogram
        parralelogram();
    }
    else if shape == "cube"{//to call the function cube if the user inputs cube
        cube();
    }
    else if shape == "cylinder"{//to call the function cylinder if the user inputs cylinder
        cylinder();
    }
    else{//outputs only if the value the users input isn't a shape
        println!("Please input a valid shape");
        println!("If your desired shape is not in the table contact services:123-7860-763");
    
    }
    println!("Thank you for trying for trying the shape calculator ");
    println!("Author: Mr. ONUORAH NICHOLAS C.");
}   //created by Author

fn trapezium()->f64{//creating a float function for trapezium calculations
    println!("NICE you chose TRAPEZIUM");
 
    //user inputs the height
    println!("Input the height:");
    let mut h =String::new();
    io::stdin().read_line(&mut h).expect("please input a valid digit");
    let h:f64= h.trim().parse().expect("Ensure that your shape is in the table above");

  //user inputs the 1st base 
    println!("Input the base1:");
    let mut b =String::new();
    io::stdin().read_line(&mut b).expect("please input a valid digit");
    let base1:f64= b.trim().parse().expect("Ensure that your shape is in the table above");

  //user inputs the 2nd base
    println!("Input the base2:");
    let mut c =String::new();
    io::stdin().read_line(&mut c).expect("please input a valid digit");
    let base2:f64= c.trim().parse().expect("Ensure that your shape is in the table above");

//the formula used in calculating the trapezium
    let trapezium = h/2.0 * (base1 + base2);
    println! ("The area of a Trapezium is : {}",trapezium);
    println!("This is the formula ,Trapezium,Area= height/2 * ({} +{})",base1,base2);
    println!("Trapezium :{}",trapezium);
    //to output the calculated trapezium value
    
    trapezium//to return back to the main function after execution

}

fn rhombus()->f64{//creating a float function for trapezium calculations
    println!("NICE you chose RHOMBUS");
    println!("Input the diagonal1:");

    //user inputs the 1st diagonal
    let mut d =String::new();
    io::stdin().read_line(&mut d).expect("please input a valid digit");
    let diagonal1:f64= d.trim().parse().expect("Ensure that your shape is in the table above");

    //user inputs the 2nd diagonal 
    println!("Input the diagonal2:");
    let mut e =String::new();
    io::stdin().read_line(&mut e).expect("please input a valid digit");
    let diagonal2:f64= e.trim().parse().expect("Ensure that your shape is in the table above");

    //the firmula used in calculating the rhombus
    let rhombus = 1.0/2.0 * diagonal1 * diagonal2;
    println!("The formula used : Rhombus,Area= 1/2 * {}* {}",diagonal1, diagonal2);
    println!("Rhombus :{}",rhombus);
    //to output the result if the parralelogram

    rhombus//to return back to the main function after execution 
}

fn parralelogram()->f64{//creating a float function for the parralelogram calculations 
    println!("NICE you chose PARRALELOGRAM");
    //to input the base
    println!("Input the base:");
    let mut bas =String::new();
    io::stdin().read_line(&mut bas).expect("please input a valid digit");
    let base:f64= bas.trim().parse().expect("Ensure that your shape is in the table above");
    
    //to input the altitude
    println!("Input the altitude:");
    let mut alt =String::new();
    io::stdin().read_line(&mut alt).expect("please input a valid digit");
    let altitude:f64= alt.trim().parse().expect("Ensure that your shape is in the table above");
     
    //the formula used in calculating the parralelogram  
    let parralelogram= base * altitude ;
    println!("The formula used : Parralelogram,Area= {} * {}",base,altitude);
    println!("Parallelogram :{}",parralelogram);
    // to output the result of the parralelogram 

    parralelogram

}

fn cube()->f64{//creating a float function for the cubical calculations 
    println!("NICE you chose CUBE");

    //to input the side
    println!("Input the side:");
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("please input a valid digit for the side");
    let side:f64= s.trim().parse().expect("Ensure that your shape is in the table above");
    
    //the formula used to solve the cube
    let cube = 6.0* side.powf(2.0);
    println!("The formula used : Cube,Surface Area= 6 * {} ^2 ",side);
    println!("cube : {}",cube);
    // to output the value of the cube
    
    cube//to return back to the main function 

}

fn cylinder()->f64{//creating a float function for the cylinder calculations
    println!("NICE you chose CYLINDER");

    //to input the radius
    println!("Input the radius: ");
    let mut r= String::new();
    io::stdin().read_line(&mut r).expect("please input a valid digit for your radius");
    let radius:f64= r.trim().parse().expect("Ensure your shape is in the table above");

    println!("input the height: ");
    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("Please input  a valid digit for your height");
    let length:f64 =m.trim().parse().expect("Ensure that your shape is in the table is in the table above");
    
    //the pie is not inputed
    let pie = 22.0/7.0;
    println!("Pie is written automatically as {}",pie);
    

    //the formula used to solve the cylinder
    let cylinder= pie* (radius.powf(2.0) )*length;
    println!("The formula used : Cylinder,Volume= {} *({}^2 ) * {} ",pie,radius,length);
    println!("cube :{}",cylinder);
    //to output the value of cylinder
    cylinder
    //to return to the main function
}
