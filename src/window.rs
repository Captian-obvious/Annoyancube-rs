#![allow(dead_code)]

use bevy::window::{Window, WindowId, WindowMode};

/// A builder for creating and configuring a Bevy window.
pub struct WindowBuilder {
    window: Window,
}
impl WindowBuilder {
    /// Creates a new `WindowBuilder` with default settings.
    pub fn new() -> Self {
        Self {
            window: Window::new(WindowId::primary()),
        }
    }

    /// Sets the title of the window.
    pub fn with_title(mut self, title: &str) -> Self {
        self.window.title = title.to_string();
        self
    }

    /// Sets the width of the window.
    pub fn with_width(mut self, width: f32) -> Self {
        self.window.width = width;
        self
    }

    /// Sets the height of the window.
    pub fn with_height(mut self, height: f32) -> Self {
        self.window.height = height;
        self
    }

    /// Sets the window mode (e.g., Windowed, Fullscreen).
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.window.mode = mode;
        self
    }

    /// Builds and returns the configured `Window`.
    pub fn build(self) -> Window {
        self.window
    }
}