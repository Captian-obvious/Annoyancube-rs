#![allow(dead_code)]
use bevy::prelude::*;

mod obj {
    #[derive(Component)]
    pub struct Obj {
        pub id: u32,
        pub name: String,
    }
    impl Obj {
        pub fn new(id: u32, name: &str) -> Self {
            Obj {
                id,
                name: name.to_string(),
            }
        }
    }
    #[derive(Obj)]
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
        pub rotation: f64,
        pub position: Vector2,
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
}