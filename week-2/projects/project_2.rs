fn main(){
//amounts
	let toshiba:f64 =450_000.00;
	let mac:f64 =1_500_000.00;
	let hp:f64 =750_000.00;
	let dell:f64 =2_850_000.00;
	let acer:f64 =250_000.00;
	/*average
	& sum */
	
	let sum = toshiba + mac + hp + dell+ acer; 
	let average = sum/2.0+1.0+3.0+3.0+1.0;   

	println!("P.M.Okeke Sons Limited");
	println!("sum ${}",sum);
	println!("average ${}",average);   


}