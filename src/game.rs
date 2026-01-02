#![allow(dead_code)]
use bevy::prelude::*;
use std::vec::Vec;
pub mod obj;
pub mod phys;
use phys::*;
use obj::*;

#[derive(Component)]
pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub position: Vector2,
    pub velocity: Vector2,
}
impl GameObject {
    pub fn new(id: u32, name: &str, position: Vector2, velocity: Vector2) -> Self {
        GameObject {
            id,
            name: name.to_string(),
            position,
            velocity,
        }
    }
}
#[derive(Component)]
pub struct Level {
    pub name: String,
    pub objects: Vec<Rectangle>,
    pub number: u32,
}
impl Level {
    pub fn new(name: &str, number: u32) -> Self {
        Level {
            name: name.to_string(),
            objects: Vec::new(),
            number,
        }
    }
    pub fn add_object(&mut self, object: Rectangle) {
        self.objects.push(object);
    }
    pub fn get_objects(&self) -> &Vec<Rectangle> {
        &self.objects
    }
    pub fn get_number(&self) -> u32 {
        self.number
    }
    pub fn get_object_count(&self) -> usize {
        self.objects.len()
    }
    pub fn get_next_id(&self) -> u32 {
        self.objects.len() as u32 + 1
    }
}
pub mod systems {
    use super::*;
    pub fn load_level(mut commands: Commands) {
        let level = Level::new("Level 1", 1);
        commands.spawn().insert(level);
    }
}

pub fn update_game_state() {
    // Placeholder for game state update logic
}