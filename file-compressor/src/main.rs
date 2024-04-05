// use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

/*
fn even_gcd(int a, int b) -> int {
    let result = min(a, b);
    while result > 0 {

    }
} */

fn read_file_to_string(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Couldn't Read File.");
    
    println!("{}", contents);
}

fn read_file_to_bytes(path: &str) {
    // let mut odd_gcd = 0;
    // let mut even_gcd = 0;
    let mut even_byte_list: Vec<u8> = Vec::new();
    let mut odd_byte_list: Vec<u8> = Vec::new();
    if let Ok(f) = File::open(path) {
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        if let Ok(_) = reader.read_to_end(&mut buffer) {
            for byte in &buffer {
                // println!("BYTE: {}", byte);
                if byte % 2 == 0 {
                    even_byte_list.push(*(byte));
                }
                if byte % 2 != 0 {
                    odd_byte_list.push(*(byte));
                }
            }
        } else {
            eprintln!("Error reading file: {}", path);
        }
    } else {
        eprintln!("Error opening file: {}", path);
    }
    // println!("Highest Odd GCD {} | Highest Even GCD {}", highest_odd_gcd, highest_even_gcd)
    println!("{:?}", odd_byte_list);
    println!("{:?}", even_byte_list);
}

/* 
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
*/

fn main() {
    println!("Hello, world!");

    let path = "../test-files/txt-to-compress.txt";

    read_file_to_string(path);
    read_file_to_bytes(path);   
    // read_file_to_bits(path);  ehh
}


// Compress if just even maybe and can figure it out based on if number is even when recieved because if it is odd it would stay that way.