#![allow(dead_code)]
use std::{fs::File, io::{self, Read, Write}, path::Path};
use std::error::Error;
use crate::game::Level;

fn serialize_level_data(level: &Level, path: &Path) -> io::Result<()> {
    // placeholder for serialization logic
}

fn deserialize_level_data() {
    // placeholder for deserialization logic
}
