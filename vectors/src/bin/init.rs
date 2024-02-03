fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(11);
    println!("{:?}", v);

    v = vec![4, 5, 6]; // re-assigning v, vec! is a macro
    println!("{:?}", v);

    v = vec![10; 5]; // semicolon is used to repeat 10 five times
    println!("{:?}", v);
}
