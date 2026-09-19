  //Rust program for quadratic roots

  use std::io;

fn main() {
    // formula for quadratic root is ax^2 + bx + c = 0

    println!("\nEnter value for a:");
    let mut s1 = String::new ();
    io::stdin().read_line(&mut s1).expect("Failed to read input");
    let mut a:f64 = s1.trim().parse().expect("Failed to input");

    println!("\nEnter value for b:");
    let mut s2 = String::new ();
    io::stdin().read_line(&mut s2).expect("Failed to read input");
    let mut b:f64 = s2.trim().parse().expect("Failed to input");

    println!("\nEnter value for c:");
    let mut s3 = String::new ();
    io::stdin().read_line(&mut s3).expect("Failed to read input");
    let mut c:f64 = s3.trim().parse().expect("Failed to input");

    // calculate discriminant,d
    // formula for discriminant is b*b - 4.0*a*c

    let d = b*b - 4.0*a*c;
    println!(" d = {}", d);

    if d > 0.0 {
        println!("d greater than zero: two distinct roots");
        let root1 = (-d + d.sqrt()) / (2.0 * d);
        let root2 = (-d - d.sqrt()) / (2.0 * d);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }
    else if d == 0.0 {
        println!("d equal to zero: exactly one real root");
        let root_1 = -d / (2.0 * d);
        println!("Root = {}", root_1);
    }
    else {
        println!("d less than zero: no real roots");
    }
     
}
