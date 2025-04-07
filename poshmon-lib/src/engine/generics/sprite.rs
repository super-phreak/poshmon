
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteData {
    id: Uuid,
    pub width: u32,
    pub height: u32,
    pub colors: u32,
    pub tile_size: u32,
    pub data: String,
    name: String,
}

impl SpriteData {
    pub fn new(width: u32, height: u32, colors: u32, tile_size: u32, data: String, name: String) -> Self{
        SpriteData { id: Uuid::new_v4(), width, height, colors, tile_size, data, name }
    }

    pub fn load(id: Uuid, width: u32, height: u32, colors: u32, tile_size: u32, data: String, name: String) -> Self{
        SpriteData { id, width, height, colors, tile_size, data, name }
    }

    pub fn get_sprite_data(&self) -> (u32, u32, String) {
        (self.width, self.height, self.data.clone())
    }
}

// impl Copy for SpriteData {

// }
