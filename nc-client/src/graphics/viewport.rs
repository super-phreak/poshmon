use std::cmp;
use super::colors::Pallet;

const TILE_SIDE_RAW: usize = 8;
//const TILE_SIZE_RAW: usize = TILE_SIDE_RAW * TILE_SIDE_RAW;

pub struct Viewport {
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    pallet: Pallet
}

impl Viewport {
    pub fn new(width: usize, height: usize, offset_x: usize, offset_y: usize, pallet: Pallet) -> Self {
        Viewport { width, height, offset_x, offset_y, pallet }
    }

    pub fn draw_canvas(&self, frame_buffer: Vec<u8>) -> String {
        let canvas_width = cmp::min(self.width, (self.width*TILE_SIDE_RAW) as usize);
        let canvas_height = cmp::min(self.height, (self.height*TILE_SIDE_RAW/2) as usize);
        let mut buffer = "".to_string();
        for row in self.offset_y..canvas_height {
            for col in self.offset_x..canvas_width {
                buffer.push_str(&self.pallet[
                    ((frame_buffer.get(((((row * 2)    ) * (self.width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap() << self.pallet.bit_depth) +
                      frame_buffer.get(((((row * 2) + 1) * (self.width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap()) as usize
                ]);
            }
            buffer.push_str("\n");
            
        }
        return buffer;
    }

    pub fn print_pallet(&self) {
        self.pallet.print_pallet();
    }
}