#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;
use super::*;
use super::phys::*;

#[derive(Component)]
pub struct Rectangle {
    p1: Vector2,
    p2: Vector2,
    rotation: f64,
}
impl Rectangle {
    pub fn create(x1: f64, y1: f64, x2: f64, y2: f64) -> Rectangle {
        Rectangle {
            p1: Vector2 { x: x1, y: y1 },
            p2: Vector2 { x: x2, y: y2 },
            rotation: 0.0,
        }
    }
    pub fn area(&self) -> f64 {
        let width = (self.p1.x - self.p2.x).abs();
        let height = (self.p1.y - self.p2.y).abs();
        width * height
    }
    pub fn perimeter(&self) -> f64 {
        let width=(self.p1.x - self.p2.x).abs();
        let height=(self.p1.y - self.p2.y).abs();
        2.0 * (width + height)
    }
    pub fn center(&self) -> Vector2 {
        Vector2 {
            x: (self.p1.x + self.p2.x) / 2.0,
            y: (self.p1.y + self.p2.y) / 2.0,
        }
    }
    pub fn draw(&self) {
        // placeholder for drawing logic
    }
}