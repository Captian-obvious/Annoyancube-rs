use std::fmt;
struct Point {
    x: f64,
    y: f64,
};
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    };
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y };
    };
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    };
    fn Display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    };
}impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.Display(f);
    };
};
// main game logic will go here, but in the meantime...
fn main() {
    println!("A position is: {}", Point { x: 10, y: 20});
};
// main.rs