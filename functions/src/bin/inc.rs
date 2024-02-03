fn main() {
    let x = increment(5);

    println!("The value of x is: {x}");
}

fn increment(x: i32) -> i32 {   
    x + 1 // No semicolon here, since we want to return the value, without using the return keyword
}
