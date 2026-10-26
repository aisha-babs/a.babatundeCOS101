fn display(v: Vec<i32>) {
    println!("Inside display {:?}", v);
}

struct Laptop {
    brand: String,
    price: u64,
}

fn calculate_purchase_cost(laptop: &Laptop, quantity: u64) -> u64 {
    laptop.price * quantity
}

fn main() {
    let inventory = vec![
        Laptop { brand: String::from("HP"), price: 650_000 },
        Laptop { brand: String::from("IBM"), price: 755_000 },
        Laptop { brand: String::from("Toshiba"), price: 550_000 },
        Laptop { brand: String::from("Dell"), price: 850_000 },
    ];

    let mut grand_total = 0;
    let quantity_to_buy = 3;

    println!("--- Purchase Breakdown (3 Laptops Each) ---");
   

    for laptop in &inventory {
        let cost = calculate_purchase_cost(laptop, quantity_to_buy);
        println!("Cost for 3 {} laptops: ₦{}", laptop.brand, cost);
        grand_total += cost;
    }

    println!("\nGrand Total Cost: ₦{}", grand_total);
}
