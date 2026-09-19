// Rust program to determine annual incentive

use std::io;

fn main() {
    // Get experience
    println!("Is the employee experienced? (yes/no):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    //Get age if experience
    let mut age = 0;
    if is_experienced {
        println!("Enter employee age: ");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        age = age_input.trim().parse().expect("Please enter a valid number");
    }
    // Calculate incentive
    let incentive = if !is_experienced {
        100_000
    } else if age >=40 {
        1_560_000
    } else if age >=30 && age <= 39 {
        1_480_000
    } else {
        // below 30 (covers below 28 from slide)
        1_300_000
    };
    println!("\nAnnual Incentive: N{}", incentive);
}
