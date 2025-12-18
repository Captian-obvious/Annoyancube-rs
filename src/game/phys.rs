#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Component)]
pub struct Vector2 {
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