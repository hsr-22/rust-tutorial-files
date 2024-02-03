fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy, expensive operation
    println!("{s1} {s2}");
}
