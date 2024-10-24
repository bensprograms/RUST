use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("Cannot create file");
    file.write_all(b"Hello, Rust!").expect("Cannot write to file");

    let mut contents = String::new();
    File::open("output.txt").expect("Cannot open file").read_to_string(&mut contents).expect("Cannot read file");
    println!("File contents: {}", contents);
}
