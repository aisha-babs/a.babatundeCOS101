fn main() {
    
    use std::io;

    println!("--- THE RESTAURANT MENU ---");
    println!("P - Poundo Yam / Edinkaiko Soup = N3,200");
    println!("F - Fried Rice & Chicken = N3,000");
    println!("A - Amala & Ewedu Soup = N2,800");
    println!("E - Eba & Egusi Soup = N2,000");
    println!("W - White Rice & Stew = N2,500");

    // Read food type
    println!("\nEnter food code (P,F,A,E,W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read");
    let food_type = food_type.trim().to_uppercase();

    // Read quantity
    println!("Enter quantity:");
    let mut qty_str = String::new();
    io::stdin().read_line(&mut qty_str).expect("Failed to read");
    let quantity: f64 = qty_str.trim().parse().expect("Enter a number");

    // Price decision using match
    let price = match food_type.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2800.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food code!");
            return;
        }
    };

    let total = price * quantity;
    
    println!("\nTotal before discount: N{:.2}", total);

    // Discount logic
    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;
        println!("You got a 5% discount of N{:.2}", discount);
        println!("Final total to pay: N{:.2}", final_total);
    } else {
        println!("No discount (Because total is less than N10,000)");
        println!("Final total to pay: N{:.2}", total);
    }
}
