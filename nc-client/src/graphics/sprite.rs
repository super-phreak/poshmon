use std::error::Error;

use poshmon_lib::engine::generics::SpriteData;
use serde::Deserialize;
use base64::{prelude::*, DecodeError};

#[derive(Debug, Clone, Deserialize)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub colors: u32,
    pub bit_depth: u32,
    pub tile_size: u32,
    data: Vec<u8>,
}

impl Sprite {
    pub fn new(width: u32, height: u32, colors: u32, tile_size: u32, data: String) -> Result<Self, Box<dyn Error>>{
        let bit_depth = u32::BITS - (colors-1).leading_zeros();
        let sprite_data = decompress_sprite(&bit_depth, &colors, data)?;
        Ok(Sprite { width, height, colors, bit_depth, tile_size, data: sprite_data })
    }

    pub fn to_vbuff(&self, flip: bool) -> Result<Vec<u8>, Box<dyn Error>>{
        let mut v_buff: Vec<u8> = vec![0;(self.height*self.width*self.tile_size*self.tile_size) as usize];
        
        for index in 0..self.height*self.tile_size*self.tile_size {
            let bound = self.width*self.tile_size;
            if let Some(bits) = self.data.get((index*bound) as usize..(index*bound+bound) as usize) {
                let mut bits = bits.to_vec();
                if flip {bits.reverse();}
                for width in 0..bound {
                    v_buff.insert(((index*bound)+width) as usize,bits[width as usize]);
                }
            }
            
        }
        return Ok(v_buff)
    }

    pub fn scale_sprite(&self, scale: u32) -> Result<Self, Box<dyn Error>> {
        let mut scaled_sprite = vec![0;(self.height*self.width*self.tile_size*self.tile_size*scale*scale) as usize];
        println!("{}", self.data.len());

        for pixel in 0..self.data.len() as u32 {
            for scale_factor_row in 0..scale {
                for scale_factor_col in 0..scale {
                    scaled_sprite[(((pixel%(self.width*self.tile_size))*scale) + scale_factor_col + ((((pixel/(self.width*self.tile_size))*scale)+scale_factor_row)*(self.width*self.tile_size*scale))) as usize] = self.data[pixel as usize];
                }
            }
        }

        Ok(Sprite{ width: self.width*scale, height: self.height*scale, colors: self.colors, bit_depth: self.bit_depth, tile_size: self.tile_size, data: scaled_sprite })
    }

    pub fn get_bounds(&self) -> (u32, u32) {
        (self.width, self.height)
    }

}

impl TryFrom<SpriteData> for Sprite {

    type Error = &'static str;

    fn try_from(sprite_data: SpriteData) -> Result<Self, Self::Error> {
        match Sprite::new(sprite_data.width, sprite_data.height, sprite_data.colors, sprite_data.tile_size, sprite_data.data) {
            Ok(sprite) => Ok(sprite),
            Err(_) => Err("Invalid Sprite Data"),
        }
    }
}

fn decompress_sprite(bit_depth: &u32, colors: &u32, data: String) -> Result<Vec<u8>, DecodeError> {
    let mut sprite_data: Vec<u8> = Vec::new();
    let sprite_bytes = BASE64_STANDARD.decode(data)?;
    for bytenum in 0..sprite_bytes.len() as u32 {
        match sprite_bytes.get(bytenum as usize) {
            Some(byte) => {
                for div in 0..*colors {
                    sprite_data.insert((bytenum * colors + div) as usize, ((*byte >> ((8-bit_depth) - (div * bit_depth))) & ((1 << bit_depth)-1)) as u8);
                }
            },
            None => todo!(),
        }

    }
    return Ok(sprite_data);
}