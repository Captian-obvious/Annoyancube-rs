#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use crate::game::*;
// AND IT BEGINS!

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
