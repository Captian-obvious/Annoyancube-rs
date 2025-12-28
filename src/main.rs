#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use crate::game::*;
use crate::window::*;
// AND IT BEGINS!
let primaryWindow=WindowBuilder::new()
    .with_title("My Bevy App")
    .with_resolution(800.0, 600.0)
    .build();
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
};
