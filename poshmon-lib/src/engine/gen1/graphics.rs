use std::{fmt::Display, error::Error};

use serde::Deserialize;
use uuid::Uuid;
use base64::DecodeError;

const TILE_SIDE_RAW: i32 = 8;
const TILE_SIZE_RAW: i32 = TILE_SIDE_RAW * TILE_SIDE_RAW;

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

    fn render_sprite(&self, flip: bool) -> Result<Vec<u8>, Box<dyn Error>>{
        let (flip_sign, flip_offest) = match flip {
            true => (-1, self.width*TILE_SIDE_RAW),
            false => (1,0),
        };
        let decompressed_sprite: Vec<u8> = self.decompress_sprite()?;
        let mut v_buff: Vec<u8> = vec![0;(self.height*self.width*TILE_SIZE_RAW) as usize];
        let CANVAS_WIDTH = self.width.clone()*TILE_SIDE_RAW;
        for index in 0..self.height*TILE_SIZE_RAW {
            let bound = self.width*TILE_SIDE_RAW;
            if let Some(bits) = decompressed_sprite.get((index*bound) as usize..(index*bound+bound) as usize) {
                let mut bits = bits.to_vec();
                if flip {bits.reverse();}
                for width in 0..bound {
                    v_buff.insert(((index*CANVAS_WIDTH)+width) as usize,bits[width as usize]);
                    //v_buff.insert((((index/(self.width*TILE_SIDE_RAW))*CANVAS_WIDTH)+(((index%(self.width*TILE_SIDE_RAW))*flip_sign)+flip_offest)) as usize,bits[width as usize]);
                }
                
            }
            
        }
        return Ok(v_buff)
    }

    pub fn print_sprite_to_term(&self) {
        let sprite = self.render_sprite(false);

        match sprite {
            Ok(sprite) => println!("{}", draw_canvas(sprite, self.height, self.width)),
            Err(_) => println!("There was a decoding error in the sprite. Please check the data"),
        }
    }

}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sprite = self.render_sprite(true);

        let sprite: String = match sprite {
            Ok(sprite) => draw_canvas(sprite, self.height, self.width),
            Err(_) => "There was a decoding error in the sprite. Please check the data".to_owned(),
        };
        write!(f, "{}:{}\n\t{}x{} tiles (flipped)\n{}", self.name, self.id.to_string(),self.height,self.width,sprite)
    }
}

fn draw_canvas(decompressed_sprite: Vec<u8>, height: i32, width: i32) -> String {
    let mut buffer = "".to_string();
    for row in 0..height*TILE_SIDE_RAW/2 {
        for col in 0..width*TILE_SIDE_RAW {
            buffer.push_str(PIXELS[
                ((decompressed_sprite.get(((((row * 2)    ) * width * TILE_SIDE_RAW) + col) as usize).unwrap() << 2) +
                  decompressed_sprite.get(((((row * 2) + 1) * width * TILE_SIDE_RAW) + col) as usize).unwrap()) as usize
            ]);
        }
        buffer.push_str("\n");
        
    }
    return buffer;
}

pub fn print_pallet() {
    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
        PIXELS[0],
        PIXELS[1],
        PIXELS[2],
        PIXELS[3],
        PIXELS[4],
        PIXELS[5],
        PIXELS[6],
        PIXELS[7],
        PIXELS[8],
        PIXELS[9],
        PIXELS[10],
        PIXELS[11],
        PIXELS[12],
        PIXELS[13],
        PIXELS[14],
        PIXELS[15],
    )
}