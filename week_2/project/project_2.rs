fn main() {
	// sales amount from the table
	let toshiba: f64 = 450_000.00;
	let mac: f64 = 1_500_000.00;
	let hp: f64 = 750_000.00;
	let dell: f64 = 2_850_000.00;
	let acer: f64 = 250_000.00;

	// Calculate sum
	let sum = toshiba + mac + hp + dell + acer;

	// Calculate average of the 5 items
	let average = sum / 5.0;

	println!("total sales sum: {}", sum);
	println!("average sales: {}", average);

}