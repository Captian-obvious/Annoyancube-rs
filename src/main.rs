#![allow(dead_code)]
use bevy::prelude::*;
mod game;
use std::fmt;
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
impl Rectangle {
    pub fn create(x1: f64, y1: f64, x2: f64, y2: f64, position: Vector2) -> Rectangle {
        Rectangle {
            width: (x2 - x1).abs(),
            height: (y2 - y1).abs(),
            rotation: 0.0,
        }
    }
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    pub fn center(&self) -> Vector2 {
        Vector2 {
            x: (self.width) / 2.0,
            y: (self.height) / 2.0,
        }
    }
    pub fn draw(&self) {
        // placeholder for drawing logic
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
