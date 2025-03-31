use std::{fmt::Display, error::Error, cmp};

use serde::Deserialize;
use uuid::Uuid;
use base64::DecodeError;

const TILE_SIDE_RAW: i32 = 8;
const TILE_SIZE_RAW: i32 = TILE_SIDE_RAW * TILE_SIDE_RAW;
//const CANVAS_WIDTH: i32 = TILE_SIDE_RAW * 10;

// struct Canvas {
//     width: i32,
//     heigth: i32,
//     tile_side_px: i32
// }

const PIXELS: [&str; 16] = [
    //White 202, 220, 159
    //LiteG 139, 172, 15 
    //DarkG 48,  98,  48 
    //Black 15,  56,  15 
    //end header \x1b[0m
    //header \x1b[

    "\x1b[38;2;202;220;159m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
    "\x1b[38;2;139;172;15m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
    "\x1b[38;2;48;98;48m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
    "\x1b[38;2;15;56;15m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",

    "\x1b[38;2;202;220;159m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
    "\x1b[38;2;139;172;15m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
    "\x1b[38;2;48;98;48m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
    "\x1b[38;2;15;56;15m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
 
    "\x1b[38;2;202;220;159m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
    "\x1b[38;2;139;172;15m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
    "\x1b[38;2;48;98;48m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
    "\x1b[38;2;15;56;15m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
    
    "\x1b[38;2;202;220;159m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
    "\x1b[38;2;139;172;15m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
    "\x1b[38;2;48;98;48m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
    "\x1b[38;2;15;56;15m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
];

#[derive(Debug, Clone, Deserialize)]
pub struct Sprite {
    id: Uuid,
    width: i32,
    height: i32,
    data: String,
    name: String,
}

impl Sprite {
    pub fn new(width: i32, height: i32, data: String, name: String) -> Self{
        Sprite { id: Uuid::new_v4(), width, height, data, name}
    }

    pub fn load(id: Uuid, width: i32, height: i32, data: String, name: String) -> Self{
        Sprite { id, width, height, data, name}
    }

    fn decompress_sprite(&self) -> Result<Vec<u8>, DecodeError> {
        let mut sprite_data: Vec<u8> = Vec::new();
        let sprite_bytes = base64::decode(self.data.clone())?;
        for bytenum in 0..sprite_bytes.len() {
            match sprite_bytes.get(bytenum) {
                Some(byte) => {
                    for div in 0..4 {
                        sprite_data.insert(bytenum * 4 + div, (*byte >> (6 - (div * 2))) & 3);
                    }
                },
                None => todo!(),
            }

        }
        //println!("sprite_map {:?}:{} -> {}",sprite_bytes[0..32],sprite_bytes.len(),self.height*self.width*TILE_SIZE_RAW);
        
        return Ok(sprite_data);
    }

    pub fn render_sprite(&self, flip: bool) -> Result<Vec<u8>, Box<dyn Error>>{
        // let (flip_sign, flip_offest) = match flip {
        //     true => (-1, self.width*TILE_SIDE_RAW),
        //     false => (1,0),
        // };
        let decompressed_sprite: Vec<u8> = self.decompress_sprite()?;
        let mut v_buff: Vec<u8> = vec![0;(self.height*self.width*64) as usize];
        
        for index in 0..self.height*8 {
            let bound = self.width*8;
            if let Some(bits) = decompressed_sprite.get((index*bound) as usize..(index*bound+bound) as usize) {
                let mut bits = bits.to_vec();
                if flip {bits.reverse();}
                for width in 0..bound {
                    v_buff.insert(((index*bound)+width) as usize,bits[width as usize]);
                }
            }
            
        }
        return Ok(v_buff)
    }

    pub fn print_sprite_to_term(&self) {
        let sprite = self.render_sprite(false);

        match sprite {
            Ok(sprite) => println!("{}", draw_canvas(sprite, self.width, self.height, Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0, 0))),
            Err(_) => println!("There was a decoding error in the sprite. Please check the data"),
        }
    }

    pub fn draw_sprite(&self, flip: bool, viewport: Option<Viewport>) -> String {
        let viewport = match viewport {
            Some(v) => v,
            None => Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0, 0)
        };

        match self.render_sprite(flip) {
            Ok(sprite) => draw_canvas(sprite, self.height, self.width, viewport),
            Err(_) => "There was an error decoding the sprite.".to_owned()
        }
    }

    pub fn get_bounds(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_sprite_data(&self) -> (i32, i32, String) {
        (self.width, self.height, self.data.clone())
    }



}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sprite = self.render_sprite(true);

        let sprite: String = match sprite {
            Ok(sprite) => draw_canvas(sprite, self.height, self.width, Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0,0)),
            Err(_) => "There was a decoding error in the sprite. Please check the data".to_owned(),
        };
        write!(f, "{}:{}\n\t{}x{} tiles (flipped)\n{}", self.name, self.id.to_string(),self.height,self.width,sprite)
    }
}

pub struct Viewport {
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
}

impl Viewport {
    pub fn new(width: usize, height: usize, offset_x: usize, offset_y: usize) -> Self {
        Viewport { width, height, offset_x, offset_y }
    }
}

fn draw_canvas(frame_buffer: Vec<u8>, height: i32, width: i32, viewport: Viewport) -> String {
    let canvas_width = cmp::min(viewport.width, (width*TILE_SIDE_RAW) as usize);
    let canvas_height = cmp::min(viewport.height, (height*TILE_SIDE_RAW/2) as usize);
    let mut buffer = "".to_string();
    println!("DEBUG: {}", frame_buffer.len());
    for row in viewport.offset_y..canvas_height {
        for col in viewport.offset_x..canvas_width {
            buffer.push_str(PIXELS[
                ((frame_buffer.get(((((row * 2)    ) * (width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap() << 2) +
                  frame_buffer.get(((((row * 2) + 1) * (width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap()) as usize
            ]);
        }
        buffer.push_str("\n");
        
    }
    return buffer;
}

pub fn print_pallet() {
    let buf: String = PIXELS.into_iter().collect();
    println!("Pallet: {}", buf)
}