use std::io::{self, Stdout};
use crossterm::{
    cursor, terminal::{Clear, ClearType}, QueueableCommand
};

use super::{Pallet, Viewport};

pub struct Canvas {
    width: usize,
    height: usize,
    tile_side: u8,
    pallet: Pallet,
    viewport: Viewport,
    canvas: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, tile_side: u8, pallet: Pallet, viewport: Viewport) -> Self {
        Canvas { width, height, tile_side, pallet, viewport, canvas: vec![0;width*height]}
    }

    pub fn render(&self, stdout: &mut Stdout) -> io::Result<()>{
        stdout.queue(Clear(ClearType::All))?.queue(cursor::MoveTo(0,0))?;
        self.viewport.draw_frame(&self.width, &self.height, &self.canvas, &self.pallet);
        Ok(())
    }

    pub fn add_raw_to_canvas(&mut self, loc_x: usize, loc_y: usize, data_width: &usize, data_height: &usize, data: &Vec<u8>) {
        // for ($index=0;$index -lt ($sprite.height*$sprite.width*$TILE_SIZE_RAW);$index++) {
        //     $v_buff[([MATH]::Floor($index/($sprite.width*$TILE_SIDE_RAW))*$CANVAS_WIDTH)+$offset+(($index%($sprite.width*$TILE_SIDE_RAW))*$FLIP_SIGN)+$FLIP_OFFSET] = $sprite.data[$index]
        // }

        for index in 0..data_height * data_width {
            self.canvas[((index/data_width)*self.width)+(loc_x+(self.width*loc_y))+(index%(data_width))] = data[index];
        }
    }

    pub fn add_tile_to_canvas(&mut self, loc_x: usize, loc_y: usize, data_width: &usize, data_height: &usize, data: &Vec<u8>) {
        self.add_raw_to_canvas(loc_x * self.tile_side as usize, loc_y * self.tile_side as usize, &(data_width*self.tile_side as usize), &(data_height*self.tile_side as usize), data);
    }

    pub fn print_pallet(&self) {
        self.pallet.print_pallet();
    }
}