use std::{fmt::Display, error::Error, cmp};

use serde::Deserialize;
use uuid::Uuid;
use base64::DecodeError;

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteData {
    id: Uuid,
    width: i32,
    height: i32,
    data: String,
    name: String,
}

impl SpriteData {
    pub fn new(width: i32, height: i32, data: String, name: String) -> Self{
        SpriteData { id: Uuid::new_v4(), width, height, data, name}
    }

    pub fn load(id: Uuid, width: i32, height: i32, data: String, name: String) -> Self{
        SpriteData { id, width, height, data, name}
    }

    pub fn get_sprite_data(&self) -> (i32, i32, String) {
        (self.width, self.height, self.data.clone())
    }
}

// impl Copy for SpriteData {

// }
