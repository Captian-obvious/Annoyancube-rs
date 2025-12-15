#![allow(dead_code)]
use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .run();
};

fn setup(mut commands: Commands) {
    println!("Hello, Bevy!");
};