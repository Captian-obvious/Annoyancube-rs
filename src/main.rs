#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowPlugin};
use bevy::sprite_render::{Wireframe2dPlugin};
mod window;
mod game;
use crate::window::*;
use crate::game::*;
// AND IT BEGINS!
fn main() {
    let window=setup_window();
    let mut window_plugin=WindowPlugin {
        primary_window: Some(window),
        ..Default::default()
    };
    let mut game_state=GameState::new(window_plugin.primary_window.as_mut().unwrap());
    game_state.set_window_title("\"Slight\" Annoyancube".to_string());
    let mut binding = App::new();
    let mut app=binding
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(
            #[cfg(not(target_arch = "wasm32"))]
            Wireframe2dPlugin::default()
        );
    game_state.add_app(&mut app);
    game_state.setup();
    app.run();
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