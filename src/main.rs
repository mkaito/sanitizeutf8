#![feature(io)]
use std::io;
use std::io::Read;

fn main() {
    let reader = io::stdin();
    for c in reader.lock().chars() {
        match c {
            Ok(character) => print!("{}", character),
            Err(_) => continue,
        }
    }
}
