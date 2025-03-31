use std::error::Error;

use serde::Deserialize;
use base64::DecodeError;

#[derive(Debug, Clone, Deserialize)]
pub struct Sprite {
    width: i32,
    height: i32,
    data: String,
}

impl Sprite {
    pub fn new(width: i32, height: i32, data: String) -> Self{
        Sprite { width, height, data }
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

    pub fn to_vbuff(&self, tile_size: i32, flip: bool) -> Result<Vec<u8>, Box<dyn Error>>{
        // let (flip_sign, flip_offest) = match flip {
        //     true => (-1, self.width*TILE_SIDE_RAW),
        //     false => (1,0),
        // };
        let decompressed_sprite: Vec<u8> = self.decompress_sprite()?;
        let mut v_buff: Vec<u8> = vec![0;(self.height*self.width*tile_size*tile_size) as usize];
        
        for index in 0..self.height*tile_size*tile_size {
            let bound = self.width*tile_size;
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

    // pub fn print_sprite_to_term(&self) {
    //     let sprite = self.render_sprite(false);

    //     match sprite {
    //         Ok(sprite) => println!("{}", draw_canvas(sprite, self.width, self.height, Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0, 0))),
    //         Err(_) => println!("There was a decoding error in the sprite. Please check the data"),
    //     }
    // }

    // pub fn draw_sprite(&self, flip: bool, viewport: Option<Viewport>) -> String {
    //     let viewport = match viewport {
    //         Some(v) => v,
    //         None => Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0, 0)
    //     };

    //     match self.render_sprite(flip) {
    //         Ok(sprite) => draw_canvas(sprite, self.height, self.width, viewport),
    //         Err(_) => "There was an error decoding the sprite.".to_owned()
    //     }
    // }

    pub fn get_bounds(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_sprite_data(&self) -> (i32, i32, String) {
        (self.width, self.height, self.data.clone())
    }



}

// impl Display for Sprite {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let sprite = self.render_sprite(true);

//         let sprite: String = match sprite {
//             Ok(sprite) => draw_canvas(sprite, self.height, self.width, Viewport::new((self.width*8) as usize, (self.height*4) as usize, 0,0)),
//             Err(_) => "There was a decoding error in the sprite. Please check the data".to_owned(),
//         };
//         write!(f, "{}:{}\n\t{}x{} tiles (flipped)\n{}", self.name, self.id.to_string(),self.height,self.width,sprite)
//     }
// }