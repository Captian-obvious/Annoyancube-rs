#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window, WindowMode};

pub struct GameState{
    score: u32,
    level: u32,
    window: *mut Window,
} impl GameState {
    pub fn new(window: &mut Window) -> Self {
        Self {
            score: 0,
            level: 1,
            window:window,
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
    // Gets the window from the reference
    pub fn get_window(&self) -> &mut Window {
        if (self.window.is_null()) {
            panic!("Window pointer is null!");
        };
        // Safety: We ensure that the window pointer is valid before dereferencing (its required to be valid for the struct to initialize)
        unsafe { &mut *self.window }
    }
    // Sets the window title of the attached window
    pub fn set_window_title(&mut self, title: String) {
        let window_ref: &mut Window = self.get_window();
        window_ref.title = title;
    }
}
