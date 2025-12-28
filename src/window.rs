#![allow(dead_code)]

mod components;
use bevy::window::{Window, WindowId, WindowMode, WindowResolution};

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

    /// Sets the resolution of the window.
    pub fn with_resolution(mut self, width, height) -> Self {
        let resolution = WindowResolution::new(width, height);
        self.window.resolution = resolution;
        self
    }

    /// Sets the window mode (e.g., Windowed, Fullscreen).
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.window.mode = mode;
        self
    }

    // Sets whether the window is resizable.
    pub fn is_resizable(mut self, resizable: bool) -> Self {
        self.window.resizable = resizable;
        self
    }

    /// Builds and returns the configured `Window`.
    pub fn build(self) -> Window {
        self.window
    }
}