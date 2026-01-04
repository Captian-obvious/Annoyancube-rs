#![allow(dead_code)]

use bevy::window::{Window, WindowMode, WindowResolution};

/// A builder for creating and configuring a Bevy window.
pub struct WindowBuilder {
    window: Window,
} impl WindowBuilder {
    /// Creates a new `WindowBuilder` with default settings.
    pub fn new() -> Self {
        Self {
            window: Window::default(),
        }
    }

    /// Sets the title of the window.
    pub fn with_title(mut self, title: &str) -> Self {
        self.window.title = title.to_string();
        self
    }

    /// Sets the resolution of the window.
    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        let resolution = WindowResolution::new(width, height);
        self.window.resolution = resolution;
        self
    }

    // Sets whether the window is resizable.
    pub fn is_resizable(mut self, resizable: bool) -> Self {
        self.window.resizable = resizable;
        self
    }

    // Sets whether the window is visible.
    pub fn is_visible(mut self, visible: bool) -> Self {
        self.window.visible = visible;
        self
    }

    // Sets the default window mode. (e.g., Windowed, Fullscreen)
    pub fn in_mode(mut self, mode: WindowMode) -> Self {
        self.window.mode=mode;
        self
    }

    /// Builds and returns the configured `Window`.
    pub fn build(self) -> Window {
        self.window
    }
}