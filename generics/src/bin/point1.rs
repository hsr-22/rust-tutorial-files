struct Point<T> { // Generic type T, type parameter
    x: T,
    y: T,
}

// Type T needs to support add and mul operations
//cargo add num-traits, cargo.toml gets modified with added dependencies
impl<T> Point<T> {
    fn square_distance_from_origin(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 5 };
    println!("Distance = {}", p.square_distance_from_origin());
}
