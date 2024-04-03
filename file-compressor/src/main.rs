// use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

fn read_file_to_string(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Couldn't Read File.");
    
    println!("{}", contents);
}

fn read_file_to_bytes(path: &str) {
    if let Ok(f) = File::open(path) {
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        if let Ok(_) = reader.read_to_end(&mut buffer) {
            for byte in &buffer {
                println!("BYTE: {}", byte);
            }
        } else {
            eprintln!("Error reading file: {}", path);
        }
    } else {
        eprintln!("Error opening file: {}", path);
    }
}

fn read_file_to_bits(path: &str) {
    if let Ok(f) = File::open(path) {
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        if let Ok(_) = reader.read_to_end(&mut buffer) {
            for byte in &buffer {
                for i in (0..8).rev() {
                    let bit = (byte >> i) & 1;
                    println!("BIT: {}", bit);
                }
            }
        } else {
            eprintln!("Error reading file: {}", path);
        }
    } else {
        eprintln!("Error opening file: {}", path);
    }
}

fn main() {
    println!("Hello, world!");

    let path = "../test-files/txt-to-compress.txt";

    read_file_to_string(path);
    read_file_to_bytes(path);   
    // read_file_to_bits(path);  ehh
}
