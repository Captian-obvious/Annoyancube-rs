struct Point {
    x: i32,
    y: i32,
    Display: fn(&self){
        format!("(X={0}, Y={1})", self.x, self.y)
    },
};
fn main() {
    println!("A position is: {}", Point { x: 10, y: 20});
};