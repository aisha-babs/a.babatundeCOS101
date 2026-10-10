use std::io;

fn main() {
    let mut choice = String::new();

   // Display the menu
    println!("SHAPE CALCULATOR");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    
    // Allow user to select a shape and read the user's choice
    println!("Enter your choice (1-5):");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: i32 = choice.trim().parse().expect("Enter a number");

    // Call an appropraite function for the user's choice
    if choice == 1 {
        trapezium();
    } else if choice == 2 {
        rhombus();
    } else if choice == 3 {
        parallelogram();
    } else if choice == 4 {
        cube();
    } else if choice == 5 {
        cylinder();
    } else {
        println!("Invalid choice!");
    }
}

// Function to calculate area of trapezium
fn trapezium() {
    let mut input = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let height: f64 = input.trim().parse().expect("Enter a number");
    input.clear();
    println!("Enter first base:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let base1: f64 = input.trim().parse().expect("Enter a number");

    input.clear();
    println!("Enter second base:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let base2: f64 = input.trim().parse().expect("Enter a number");
    let area = height / 2.0 * (base1 + base2);

    println!("Area of trapezium = {}", area);
}

// Function to calculate area of rhombus
fn rhombus() {
    let mut input = String::new();
    println!("Enter first diagonal:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let d1: f64 = input.trim().parse().expect("Enter a number");

    input.clear();
    println!("Enter second diagonal:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let d2: f64 = input.trim().parse().expect("Enter a number");
    let area = 0.5 * d1 * d2;

    println!("Area of rhombus = {}", area);
}

// Function to calculate area of parallelogram
fn parallelogram() {
    let mut input = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let base: f64 = input.trim().parse().expect("Enter a number");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let height: f64 = input.trim().parse().expect("Enter a number");
    let area = base * height;

    println!("Area of parallelogram = {}", area);
}
// Function to calculate area of cube
fn cube() {
    let mut input = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let side: f64 = input.trim().parse().expect("Enter a number");
    let area = 6.0 * side * side;

    println!("Surface area of cube = {}", area);
}

// Function to calculate area of cylinder
fn cylinder() {
    let mut input = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let radius: f64 = input.trim().parse().expect("Enter a number");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    let height: f64 = input.trim().parse().expect("Enter a number");
    let pi = 3.14159;
    let volume = pi * radius * radius * height;

    println!("Volume of cylinder = {}", volume);
}