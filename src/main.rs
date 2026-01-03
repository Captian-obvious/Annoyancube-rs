#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::*;
use std::fmt;
mod window;
use crate::window::*;
// AND IT BEGINS!
const PRIMARY_WINDOW:Window=WindowBuilder::new()
    .with_title("Annoyancube")
    .with_resolution(800, 600)
    .build();
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, game_update)
        .run();
}

fn setup() {
    println!("Hello, Bevy!");
    PRIMARY_WINDOW.show();
}
fn game_update() {
    // Game logic goes here
}
