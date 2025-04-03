use std::cmp;

use super::colors::Pallet;

//const TILE_SIDE_RAW: usize = 8;
//const TILE_SIZE_RAW: usize = TILE_SIDE_RAW * TILE_SIDE_RAW;

pub struct Viewport {
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    vbuff: Vec<u8>
}

impl Viewport {
    pub fn new(width: usize, height: usize, offset_x: usize, offset_y: usize) -> Self {
        Viewport { width, height, offset_x, offset_y, vbuff: vec![0; width*height] }
    }

    pub fn set_vbuff(&mut self, data: Vec<u8>) {
        self.vbuff = data.clone();
    }

    pub fn draw_frame(&self, canvas_width: &usize, canvas_height: &usize, canvas: &Vec<u8>, pallet: &Pallet) {
        let view_width = *cmp::min(&self.width, canvas_width);
        let view_height = *cmp::min(&self.height, &((canvas_height/2) as usize));
        
        let mut buffer = "".to_string();
        for row in self.offset_y..view_height {
            for col in self.offset_x..view_width {
                buffer.push_str(&pallet[
                    ((canvas.get((((row * 2)    ) * canvas_width) + col).unwrap_or(&0) << pallet.bit_depth) +
                      canvas.get((((row * 2) + 1) * canvas_width) + col).unwrap_or(&0)) as usize
                ]);
            }
            buffer.push_str("\n");
            
        }
        println!("{}", buffer);
    }
}