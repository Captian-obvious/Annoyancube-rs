#![allow(dead_code)]
use bevy::prelude::*;
mod game;
use std::fmt;
use game::*;
// AND IT BEGINS!

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
