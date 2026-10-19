use std::fs::OpenOptions;
use std::io::Write;

fn main () {
    let mut _file = std::fs::File::create("data.txt").expect("create failed");
    let mut _file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    _file.write_all("\nHello Class".as_bytes()).expect("write failed");
    _file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("write failed");
    println!("file append success");
}