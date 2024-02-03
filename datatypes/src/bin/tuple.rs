fn main() {
    let x: (u8, f32, bool) = (38, 5.5, true);
    // tuple is just a convenient way to group values together, might not always work
    // Note the {:?}
    println!("{:?}", x); // :? is a debug formatter, try harder to print the value
    println!("{} {} {}", x.0, x.1, x.2);
    // let i = 2;
    // println!("{}", x.i); // This will not work as i is not a field of the tuple
}
