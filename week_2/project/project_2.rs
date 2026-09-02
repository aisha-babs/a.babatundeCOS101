fn main() {
	// sales amount from the table
	let toshiba: f64 = 450_000.00; let qty_toshiba = 2.0;
	let mac: f64 = 1_500_000.00; let qty_mac = 1.0;
	let hp: f64 = 750_000.00; let qty_hp = 3.0;
	let dell: f64 = 2_850_000.00; let qty_dell = 3.0;
	let acer: f64 = 250_000.00; let qty_acer = 1.0;


    let total_toshiba = qty_toshiba * toshiba;
    let total_mac = qty_mac * mac;
    let total_hp = qty_hp * hp;
    let total_dell = qty_dell * dell;
    let total_acer = qty_acer * acer;

	
	// Calculate sum
	let sum = total_toshiba + total_mac + total_hp + total_dell + total_acer;
	// Calculate average of the 5 items
	let average = sum / 10.0;

	println!("total sales sum: {}", sum);
	println!("average sales: {}", average);

}