#![allow(dead_code)]
use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;
#[derive(Debug, Clone,Eq,Copy,PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
};
impl Point {
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    };
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y };
    };
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    };
    pub fn Display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    };
};
#[derive(Debug, Clone,Eq,Copy,PartialEq)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
    rotation: f64,
};
impl Rectangle {
    pub fn area(&self) -> f64 {
        let width = (self.p1.x - self.p2.x).abs();
        let height = (self.p1.y - self.p2.y).abs();
        width * height
    };
    pub fn perimeter(&self) -> f64 {
        let width=(self.p1.x - self.p2.x).abs();
        let height=(self.p1.y - self.p2.y).abs();
        2.0 * (width + height)
    };
    pub fn onCollide(&self, other: &Rectangle) -> bool {
        !(self.p1.x > other.p2.x || self.p2.x < other.p1.x ||
          self.p1.y > other.p2.y || self.p2.y < other.p1.y)
    };
    pub fn draw(&self) {
        // placeholder for drawing logic
    };
};
#[derive(Debug, Clone,Eq,Copy,PartialEq)]
pub struct Cube {
    rect: Rectangle,
    velocity: Point,
    health: f64,
};
impl Cube { //this is the player character, also its a square
    pub static fn create(x: f64, y: f64, size: f64) -> Cube {
        Cube {
            rect: Rectangle {
                p1: Point { x: x, y: y },
                p2: Point { x: x + size, y: y + size },
                rotation: 0.0,
            },
            velocity: Point { x: 0.0, y: 0.0 },
            health: 100.0,
        }
    };
    pub fn update(&mut self, dt: f64) {
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
    pub fn updateCollision(&mut self, other: &Rectangle) {
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
#[derive(Debug, Clone)]
pub struct Level {
    obstacles: Vec<Rectangle>,
    enemies: Vec<Cube>,
    width: u32,
    height: u32,
    lvlNum: u64,
    player: Cube,
};
impl Level {
    pub fn scaleToScreen(&mut self, screen_width: u32, screen_height: u32) {
        let scale_x = screen_width as f64 / self.width as f64;
        let scale_y = screen_height as f64 / self.height as f64;
        for obstacle in &mut self.obstacles {
            obstacle.p1.x *= scale_x;
            obstacle.p1.y *= scale_y;
            obstacle.p2.x *= scale_x;
            obstacle.p2.y *= scale_y;
        };
        for enemy in &mut self.enemies {
            enemy.rect.p1.x *= scale_x;
            enemy.rect.p1.y *= scale_y;
            enemy.rect.p2.x *= scale_x;
            enemy.rect.p2.y *= scale_y;
        };
        self.player.rect.p1.x *= scale_x;
        self.player.rect.p1.y *= scale_y;
        self.player.rect.p2.x *= scale_x;
        self.player.rect.p2.y *= scale_y;
    };
    pub fn update(&mut self, dt: f64) {
        for enemy in &mut self.enemies {
            enemy.update(dt);
        };
        self.player.update(dt);
    };
    pub fn handleCamera(&self, camera_pos: &Point) {
        for obstacle in &self.obstacles {
            // adjust obstacle position based on camera_pos
            obstacle.p1.x -= camera_pos.x;
            obstacle.p1.y -= camera_pos.y;
            obstacle.p2.x -= camera_pos.x;
            obstacle.p2.y -= camera_pos.y;
        };
        for enemy in &self.enemies {
            enemy.rect.p1.x -= camera_pos.x;
            enemy.rect.p1.y -= camera_pos.y;
            enemy.rect.p2.x -= camera_pos.x;
            enemy.rect.p2.y -= camera_pos.y;
        };
        self.draw();
    };
    pub fn draw(&self) {
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

/*
level serialization and deserialization functions
JSON from level struct to string and back
*/
fn serialize_level(level: &Level) -> String {
    // placeholder for serialization logic
    String::from("serialized_level_data")
};

fn deserialize_level(data: &str) -> Level {
    // placeholder for deserialization logic
    Level {
        obstacles: Vec::new(),
        enemies: Vec::new(),
        width: 100,
        height: 100,
        lvlNum: 1,
        player: Cube::create(0.0, 0.0, 10.0),
    }
};