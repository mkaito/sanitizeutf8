use std::io;

fn main() {
	let mut reader = io::stdin();
	for c in reader.lock().chars() {
		match c {
			Ok(character) => print!("{}", character),
			Err(_) => continue,
		}
	}
}
