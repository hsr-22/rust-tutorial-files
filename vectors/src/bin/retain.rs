#![allow(unused)]
fn main() {
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0); // retain elements that are even. 
    assert_eq!(vec, [2, 4]); 
}
