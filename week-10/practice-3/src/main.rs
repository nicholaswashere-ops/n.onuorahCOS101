fn main() {
    let v = vec![20,40,60,80];
    //vector v owns the object in heap ; because vectors are stored in the heap

    let v2 = v ;
    //v2 now owns v
    let v2_return =display(v2);
    println!("In main {:?}",v );//this line would cause an error
    //v2_return sends v2 which is v to the function display 
}

fn display(v:Vec<i32>)->Vec<i32>{
    //returning the same vector
    println!("Inside display {:?}",v);
    return v;
}

