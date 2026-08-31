fn main(){
//amounts
	let toshiba:f64 =450000.00;
	let mac:f64 =1500000.00;
	let hp:f64 =750000.00;
	let dell:f64 =2850000.00;
	let acer:f64 =250000.00;
	/*average
	& sum */
	
	let sum = toshiba + mac + hp + dell+ acer; 
	let average = sum/2.0+1.0+3.0+3.0+1.0;   

	println!("P.M.Okeke Sons Limited");
	println!("sum ${}",sum);
	println!("average ${}",average);   


}