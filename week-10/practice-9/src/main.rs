//define dimensiions of rectangle
struct Rectangle{
    width:u32, height:u32
}
//logic to calculate area of a rectangle ; uses a method with a  functions placed inside it
impl Rectangle {
    fn area(&self)->u32{
        //use the operator to fetch the value of a field via the self keyword
        self.width*self.height
    }
}
fn main() {
    //instantiate or initialize the Structure
    let small = Rectangle{
        width:10,
        height:20
    };

    //print the rectangle's area 
    println!("The width is {}\n Height is {}\n Area of Rectangleis {}",small.width,small.height,small.area());


}
