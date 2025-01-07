use std::io;


fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("enter the value a:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a:f32 =input1.trim().parse().expect("Please enter a valid number");

	println!("enter the value a:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b:f32 =input2.trim().parse().expect("Please enter a valid number");

	println!("enter the value a:");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let c:f32 =input3.trim().parse().expect("Please enter a valid number");

	// to calculate the discriminant
	let discriminant = b.powf(2.0) - 4.0 * a * c;

	// check the discriminants and calculate roots
	if discriminant > 0.0 {
		let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
		let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
		println!("The roots are real and distinct:");
		println!("Root 1 = {}", root1);
		println!("Root 2 = {}", root2);
	} else if discriminant ==0.0 {
		let root = -b / (2.0 *a);
		println!("The roots are real and equal:");
		println!("Root = {}", root);
	} else {
		println!("The roots are complex and cannot be calculated with this program.");
	}
}