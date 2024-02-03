use num_traits::Num;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Num + Copy, // T needs to support the Num trait and the Copy trait
{
    fn square_distance_from_origin(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 5 };
    println!("Distance = {}", p.square_distance_from_origin());
}
