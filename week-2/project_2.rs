fn main() { 
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	// average
	let s = t * 2.0 + ( m * 1.0) + (h * 3.0) + ( d * 3.0) + ( a * 1.0);
	println!("Amount is {}", s);
	let ave = s / 10.0;
	println!("Average is {}", ave);
}