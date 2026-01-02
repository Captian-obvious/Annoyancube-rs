#![allow(dead_code)]
use bevy::prelude::*;
use std::fmt;
use crate::game::*;
use crate::window::*;
// AND IT BEGINS!
let mut primaryWindow=WindowBuilder::new()
    .with_title("Annoyancube")
    .with_resolution(800.0, 600.0)
    .build();
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, gameUpdate)
        .run();
};

fn setup() {
    println!("Hello, Bevy!");
    primaryWindow.show();
};

fn gameUpdate(){
    update_game_state();
};
