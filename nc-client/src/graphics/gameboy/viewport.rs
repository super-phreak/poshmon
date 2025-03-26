const TILE_SIDE_RAW: i32 = 8;
const TILE_SIZE_RAW: i32 = TILE_SIDE_RAW * TILE_SIDE_RAW;


pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Pallet {
    bit_depth: u8
    colors: Vec<Color>
}

impl Pallet {
    pub fn new(bit_depth: usize) -> Self {
        Pallet {bit_depth, Vec::with_capacity(i32::pow(2,bit_depth))}
    }

    pub fn add_color(&self, color: Color) -> Result<(), Box<Error>> {
        self.colors.
    }
}
// const GB_CLASSIC_PIXELS: [&str; 16] = [
//     //White 202, 220, 159
//     //LiteG 139, 172, 15 
//     //DarkG 48,  98,  48 
//     //Black 15,  56,  15 

//     "\x1b[38;2;202;220;159m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
//     "\x1b[38;2;139;172;15m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
//     "\x1b[38;2;48;98;48m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",
//     "\x1b[38;2;15;56;15m\x1b[48;2;202;220;159m\u{2584}\x1b[0m",

//     "\x1b[38;2;202;220;159m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;139;172;15m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;48;98;48m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;15;56;15m\x1b[48;2;139;172;15m\u{2584}\x1b[0m",
 
//     "\x1b[38;2;202;220;159m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
//     "\x1b[38;2;139;172;15m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
//     "\x1b[38;2;48;98;48m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
//     "\x1b[38;2;15;56;15m\x1b[48;2;48;98;48m\u{2584}\x1b[0m",
    
//     "\x1b[38;2;202;220;159m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;139;172;15m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;48;98;48m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
//     "\x1b[38;2;15;56;15m\x1b[48;2;15;56;15m\u{2584}\x1b[0m",
// ];

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
        let canvas_width = cmp::min(width, (width*TILE_SIDE_RAW) as usize);
        let canvas_height = cmp::min(height, (height*TILE_SIDE_RAW/2) as usize);
        let mut buffer = "".to_string();
        for row in offset_y..canvas_height {
            for col in offset_x..canvas_width {
                buffer.push_str(pallet[
                    ((frame_buffer.get(((((row * 2)    ) * (width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap() << self.pallet.bit_depth) +
                      frame_buffer.get(((((row * 2) + 1) * (width * TILE_SIDE_RAW) as usize) + col) as usize).unwrap()) as usize
                ]);
            }
            buffer.push_str("\n");
            
        }
        return buffer;
    }

    pub fn print_pallet(&self) {
        let buf: String = pallet.into_iter().collect();
        println!("Pallet: {}", buf)
    }
}

