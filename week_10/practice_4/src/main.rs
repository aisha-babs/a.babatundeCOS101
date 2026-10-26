fn main() {
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
    
    // Pass a reference using & to borrow v
    print_vector(&v);
    
    // v is still available here!
    println!("{}", v[0]); 
}

// Update the parameter to accept a reference (&Vec<i32>)
fn print_vector(x: &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
