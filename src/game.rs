#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window};

#[derive(Default)]
pub struct GameState{
    score: u32,
    level: u32,
    window: *mut Window,
    has_started: bool,
    pub app: Option<*mut App>,
} impl GameState {
    pub fn new(window: &mut Window) -> Self {
        Self {
            score: 0,
            level: 1,
            window:window,
            ..Default::default()
        }
    }

    pub fn add_app(&mut self,app:&mut App){
        self.app=Some(app);
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
    fn get_window(&self) -> &mut Window {
        if self.window.is_null() {
            panic!("Window pointer is null!");
        };
        // Safety: We ensure that the window pointer is valid before dereferencing (its required to be valid for the struct to initialize)
        unsafe { &mut *self.window }
    }

    // returns the currently attached app
    fn get_app(&self) -> &mut App {
        if self.app.is_none() {
            panic!("App pointer is null!");
        };
        // Safety: We ensure that the app pointer is valid before dereferencing (its required to be valid for the struct to initialize)
        unsafe { &mut **self.app.as_ref().unwrap() }
    }
    // Sets the window title of the attached window
    pub fn set_window_title(&mut self, title: String) {
        let window_ref: &mut Window = self.get_window();
        window_ref.title = title;
    }

    // Gets the current score
    pub fn get_score(&self) -> u32 {
        self.score
    }

    // Gets the current level
    pub fn get_level(&self) -> u32 {
        self.level
    }

    // Initializes canvas of window
    pub fn initialize_canvas(&mut self) {
        let _window_ref: &mut Window = self.get_window();
        // canvas
    }
    fn setup_system(mut commands: Commands,mut _meshes: ResMut<Assets<Mesh>>,mut _materials: ResMut<Assets<ColorMaterial>>) {
        // pre-window setup logic
        commands.spawn(Camera2d);

    }
    fn on_update_system(mut _commands: Commands,mut _meshes: ResMut<Assets<Mesh>>,mut _materials: ResMut<Assets<ColorMaterial>>) {
        // Game logic
        
    }
    pub fn setup(&mut self) {
        let app_ref: &mut App = self.get_app();
        app_ref.add_systems(Startup, Self::setup_system);
        app_ref.add_systems(Update, Self::on_update_system);
    }
    pub fn start_game(&mut self){
        if self.has_started {
            return;
        } else {
            self.has_started=true;
            // Start game logic
        }
    }
}