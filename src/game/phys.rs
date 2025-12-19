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

#[derive(Component)]
pub struct PhysicsBody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f64,
}
impl PhysicsBody {
    pub fn new(position: Vector2, velocity: Vector2, mass: f64) -> PhysicsBody {
        PhysicsBody {
            position: position,
            velocity: velocity,
            mass: mass,
        }
    }
    pub fn apply_force(&mut self, force: &Vector2, delta_time: f64) {
        let acceleration = Vector2 {
            x: force.x / self.mass,
            y: force.y / self.mass,
        };
        self.velocity.x += acceleration.x * delta_time;
        self.velocity.y += acceleration.y * delta_time;
    }
    pub fn update_position(&mut self, delta_time: f64) {
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f64,
}
impl Collider {
    pub fn new(radius: f64) -> Collider {
        Collider { radius: radius }
    }
    pub fn is_colliding(&self, other: &Collider, pos_a: &Vector2, pos_b: &Vector2) -> bool {
        let distance = pos_a.distance(pos_b);
        distance < (self.radius + other.radius)
    }
}