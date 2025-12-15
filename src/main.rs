#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
struct Vector2 {
    x: f64,
    y: f64,
};
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
