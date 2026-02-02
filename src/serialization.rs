#![allow(dead_code)]
use std::{fs::File, io::{self, Read, Write}, path::Path};
use std::error::Error;
mod game;
use crate::game::Level;

fn serialize_level_data(level: &Level, path: &Path) -> io::Result<()> {
    // placeholder for serialization logic
}

fn deserialize_level_data() {
    // placeholder for deserialization logic
}

pub struct Serializer;
impl Serializer {
    pub fn save_level(level: &Level, path: &Path) -> Result<(), Box<dyn Error>> {
        serialize_level_data(level, path)?;
        Ok(())
    }
    pub fn load_level(path: &Path) -> Result<Level, Box<dyn Error>> {
        let level = deserialize_level_data()?;
        Ok(level)
    }
}