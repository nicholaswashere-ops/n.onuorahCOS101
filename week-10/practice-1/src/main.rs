fn main() {
    let v = vec![101,250,330,400];
//vector v owns the object in the heap

//only a single variable owns the heap memory at any given time
    let v2 = v;
    /*here two variables owns OWNS HEAAP VALUE,
    TWO pointers to the same content is not allowed in rust*/

    /*RUST IS very smart in terms of memory access , so it detects arace condition 
    as two variables point to same heap*/
    println!("{:?}",v )
    //this code will be an error because it is owned by v2
    

}
