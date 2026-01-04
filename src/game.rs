#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window, WindowMode};

pub struct GameState{
    score: u32,
    level: u32,
    window: Option<Window>
} impl GameState {
    pub fn new(window: &mut Window) -> Self {
        Self {
            score: 0,
            level: 1,
            window:Some(window.clone()),
        }
    }
    // Increases the score by a given number of points
    pub fn increase_score(&mut self, points: u32) {
        self.score += points;
    }
    // Advances to the next level
    pub fn next_level(&mut self) {
        self.level += 1;
    }
    // Sets the window title of the attached window
    pub fn set_window_title(&mut self, title: String) {
        let window: &mut Window=self.window.as_mut().unwrap();
        window.title = title;
    }
}
