struct Point {
    x: i32,
    y: i32,
    Display: fn(&self){
        format!("({}, {})", self.x, self.y)
    },
};
fn main() {
    println!("A position is: {}", Point { x: 10, y: 20});
}
