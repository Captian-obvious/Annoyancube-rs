#![allow(dead_code)]
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
        self.Display(f)
    };
};
struct Rectangle {
    p1: Point,
    p2: Point,
};
impl Rectangle {
    fn area(&self) -> f64 {
        let width = (self.p1.x - self.p2.x).abs();
        let height = (self.p1.y - self.p2.y).abs();
        width * height
    };
    fn perimeter(&self) -> f64 {
        let width=(self.p1.x - self.p2.x).abs();
        let height=(self.p1.y - self.p2.y).abs();
        2.0 * (width + height)
    };
    fn onCollide(&self, other: &Rectangle) -> bool {
        !(self.p1.x > other.p2.x || self.p2.x < other.p1.x ||
          self.p1.y > other.p2.y || self.p2.y < other.p1.y)
    };
    fn draw(&self) {
        // placeholder for drawing logic
    };
};
struct Cube {
    rect: Rectangle,
    velocity: Point,
    health: f64,
};
impl Cube { //this is the player character, also its a square
    fn update(&mut self, dt: f64) {
        // update position based on velocity, gravity, wind resistance, etc.
        self.rect.p1.x += self.velocity.x * dt;
        self.rect.p1.y += self.velocity.y * dt;
        self.rect.p2.x += self.velocity.x * dt;
        self.rect.p2.y += self.velocity.y * dt;
        // apply gravity
        self.velocity.y -= gravity() * dt;
        // apply wind resistance
        self.velocity.x *= wind_resistance().powf(-dt);
        self.velocity.y *= wind_resistance().powf(-dt);
        // check for collisions with ground (example rectangle)
        self.updateCollision(&Rectangle {
            p1: Point { x: -10.0, y: -10.0 },
            p2: Point { x: 10.0, y: 0.0 },
        });
        // check if we are offscreen (based on level size, for example)
        if self.rect.p1.y > 100.0 {
            self.health = 0.0; // player dies if they fall offscreen (and game resets >:) )
        };
        // draw the cube
        self.rect.draw();
    };
    fn updateCollision(&mut self, other: &Rectangle) {
        if self.rect.onCollide(other) {
            // collision response: stop movement depending on direction
            if self.velocity.y < 0.0 {
                self.velocity.y = 0.0;
                // reposition cube on top of the other rectangle
                self.rect.p1.y = other.p2.y;
                self.rect.p2.y = other.p2.y + (self.rect.p2.y - self.rect.p1.y);
            }else if self.velocity.x != 0.0 {
                self.velocity.x = 0.0;
            };
        };
    };
};
struct level {
    obstacles: Vec<Rectangle>,
    enemies: Vec<Cube>,
    width: u32,
    height: u32,
    lvlNum: u64,
    player: Cube,
};
impl level {
    fn update(&mut self, dt: f64) {
        for enemy in &mut self.enemies {
            enemy.update(dt);
        };
        self.player.update(dt);
    };
    fn draw(&self) {
        for obstacle in &self.obstacles {
            obstacle.draw();
        };
        for enemy in &self.enemies {
            enemy.rect.draw();
        };
    };
};
// main game logic will go here, but in the meantime...
fn main() {
    println!("A position is: {}", Point { x: 10.0, y: 20.0});
};
fn gravity() -> f64 {
    9.8
};
fn wind_resistance() -> f64 {
    1.2
};
// main.rs