#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;

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

mod physics {
    use super::Vector2;
    use bevy::prelude::*;
    use std::collections::HashMap;
    #[derive(Component)]
    pub struct PhysicsObject {
        pub position: Vector2,
        pub velocity: Vector2,
        pub mass: f64,
        pub is_static: bool,
    }

    impl PhysicsObject {
        pub fn new(position: Vector2, velocity: Vector2, mass: f64, is_static: bool) -> PhysicsObject {
            PhysicsObject {
                position,
                velocity,
                mass,
                is_static,
            }
        }
    }

    pub struct PhysicsEngine {
        pub objects: HashMap<u32, PhysicsObject>,
        pub next_id: u32,
    }

    impl PhysicsEngine {
        pub fn new() -> PhysicsEngine {
            PhysicsEngine {
                objects: HashMap::new(),
                next_id: 0,
            }
        }

        pub fn add_object(&mut self, obj: PhysicsObject) -> u32 {
            let id = self.next_id;
            self.objects.insert(id, obj);
            self.next_id += 1;
            id
        }

        pub fn update(&mut self, delta_time: f64) {
            for obj in self.objects.values_mut() {
                if !obj.is_static {
                    obj.position.x += obj.velocity.x * delta_time;
                    obj.position.y += obj.velocity.y * delta_time;
                }
            }
        }
    }
}