#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window};
use std::vec::Vec;

pub struct Controller{
    key_code:i32,
}impl Controller{
    pub fn new(&self){
        Self {
            key_code:0,
        }
    }
}