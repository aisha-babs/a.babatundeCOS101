fn add_one(e: &mut i32) {
    *e+= 1;
}

fn main() {
    Let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}
