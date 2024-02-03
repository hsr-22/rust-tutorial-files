#[derive(Debug)] // so we can inspect the state of our struct instances for debugging
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementation block, where we define methods on the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}