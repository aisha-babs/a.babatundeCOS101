fn main () {
	let p: f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let n: f64 = 5.0;

	// compound interest = A = P(1 + (r /100.0))^n
	let a = p * (1.0 + (r / 100.0))^n;
	// compound interest = amount - principal
	let compound_interest = a - p
    println!("Amount after 5 years is {}",a);
    println!("Compound interest is {}", ci)

}