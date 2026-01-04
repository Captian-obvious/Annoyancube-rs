#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowPlugin};
mod window;
use crate::window::*;
// AND IT BEGINS!
fn main() {
    let mut window_plugin=WindowPlugin {
        primary_window: Some(setup_window()),
        ..Default::default()
    };
    window_plugin.primary_window.as_mut().unwrap().title="\"Slight\" Annoyancube".to_string();
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, game_update)
        .add_plugins(DefaultPlugins.set(window_plugin))
        .run();
}

fn setup() {
    // pre-window setup logic goes here
}
fn setup_window()->Window {
    let primary_window:Window=WindowBuilder::new()
    .with_title("Annoyancube")
    .with_resolution(800, 600)
    .is_visible(true)
    .in_mode(WindowMode::Windowed)
    .build();
    primary_window
}

fn game_update() {
    // Game logic goes here
}