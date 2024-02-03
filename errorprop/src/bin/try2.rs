use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {

    let mut username_file = File::open("hello.txt")?; // ? operator returns the error value from the current function for the caller to handle
    // Shortcut for propagating errors: the ? operator
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn main() {
    let uname = read_username_from_file().unwrap();
    println!("{uname}");
}
