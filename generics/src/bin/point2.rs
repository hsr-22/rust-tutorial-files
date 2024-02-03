use num_traits::Num; //using the just installed num-traits crate in cargo.toml

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Num, // T needs to support the Num trait
{
    fn square_distance_from_origin(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 5 };
    println!("Distance = {}", p.square_distance_from_origin());
}
