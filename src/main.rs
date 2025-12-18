#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
struct Vector2 {
    x: f64,
    y: f64,
}
impl Vector2 {
    pub fn origin() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    pub fn distance(&self, other: &Vector2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn offset(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn Display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    }
}
impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.Display(f)
    }
}
// AND IT BEGINS!
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Position(Vector2);
#[derive(Component)]
struct Velocity(Vector2);
#[derive(Component)]
struct Acceleration(Vector2);
#[derive(Component)]
struct Mass(f64);
#[derive(Component)]
struct Rectangle {
    width: f64,
    height: f64,
    rotation: f64,
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
