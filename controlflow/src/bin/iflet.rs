fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // If i put a semicolon after 5, then the type of number will be ()

    println!("The value of number is: {number}");
}