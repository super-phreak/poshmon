use super::colors::Pallet;

//const TILE_SIDE_RAW: usize = 8;
//const TILE_SIZE_RAW: usize = TILE_SIDE_RAW * TILE_SIDE_RAW;

pub struct Viewport {
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    tile_side: usize,
    pallet: Pallet,
    vbuff: Vec<u8>
}

impl Viewport {
    pub fn new(width: usize, height: usize, offset_x: usize, offset_y: usize, tile_side: usize, pallet: Pallet) -> Self {
        Viewport { width, height, offset_x, offset_y, pallet, tile_side, vbuff: vec![0; width*height] }
    }

    pub fn set_vbuff(&mut self, data: Vec<u8>) {
        self.vbuff = data.clone();
    }

    pub fn draw_canvas(&self) {
        //let canvas_width = cmp::min(self.width, (self.width*self.tile_side) as usize);
        //let canvas_height = cmp::min(self.height, (self.height*self.tile_side/2) as usize);
        let mut buffer = "".to_string();
        for row in self.offset_y..self.height * self.tile_side / 2 {
            for col in self.offset_x..self.width * self.tile_side {
                buffer.push_str(&self.pallet[
                    ((self.vbuff.get((((row * 2)    ) * (self.width * self.tile_side)) + col).unwrap_or(&0) << self.pallet.bit_depth) +
                      self.vbuff.get((((row * 2) + 1) * (self.width * self.tile_side)) + col).unwrap_or(&0)) as usize
                ]);
            }
            buffer.push_str("\n");
            
        }
        println!("{}", buffer);
    }



    pub fn print_pallet(&self) {
        self.pallet.print_pallet();
    }
}