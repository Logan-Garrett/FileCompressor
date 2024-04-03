// use std::env;
use std::fs;

fn read_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Couldn't Read File.");
    
    println!("{}", contents);
}

fn main() {
    println!("Hello, world!");

    let path = "../test-files/txt-to-compress.txt";
    read_file(path);    
}
