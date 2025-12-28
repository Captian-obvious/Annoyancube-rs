#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
pub struct WindowComponent {
    pub title: String,
    pub width: f32,
    pub height: f32,
}
#[derive(Component)]
pub struct ButtonComponent {
    pub label: String,
    pub action: String,
}
#[derive(Component)]
pub struct TextComponent {
    pub content: String,
    pub font_size: f32,
}