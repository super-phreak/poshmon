use std::{error::Error, fmt::Display};
use std::ops::Index;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
pub struct Pallet {
    pub total_colors: u16,
    pub bit_depth: u8,
    colors: Vec<Color>,
    pallet: Vec<String>,
}

impl Pallet {
    // This function creates 
    pub fn new(total_colors: usize) -> Result<Self, Box<dyn Error>> {
        let tc: Result<u16, Box<dyn Error>> = match total_colors {
            2..=256 => {
                match u16::try_from(total_colors) {
                    Ok(tc) => Ok(tc),
                    Err(err) => Err(Box::new(err)),
                }
            },
            r => Err(Box::new(PalletErrors::InvalidNumberOfColors(r))),
        };

        let bd: Result<u8, Box<dyn Error>> = match u8::try_from(usize::BITS - (total_colors-1).leading_zeros()) {
            Ok(bd) => Ok(bd),
            Err(err) => Err(Box::new(err)),
        };

        
        match (tc, bd) {
            (Ok(tc), Ok(bd)) => {
                Ok(Pallet {total_colors: tc, bit_depth: bd, colors: vec![Color::default(); total_colors], pallet: vec!["".to_owned(); total_colors*total_colors]})
            },
            (_, Err(err)) | (Err(err), _)=> Err(err),
        }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        self.colors[index] = color;
        self.build_pallet();
    }

    fn build_pallet(&mut self) {
        for top in 0..self.total_colors {
            for bottom in 0..self.total_colors {
                self.pallet[((top << self.bit_depth)+bottom) as usize] = format!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2584}\x1b[0m", self.colors[bottom as usize].r, self.colors[bottom as usize].g, self.colors[bottom as usize].b, self.colors[top as usize].r, self.colors[top as usize].g, self.colors[top as usize].b)
            }
        }
    }

    pub fn print_pallet(&self) {
        let buf: String = self.pallet.clone().into_iter().collect();
        println!("Pallet: {}", buf)
    }

}

impl Index<usize> for Pallet {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.pallet[index].as_str()
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum BulitinPallets {
    GbClassicColors,
}

pub struct Pallets {
    pallets: HashMap<BulitinPallets, Pallet>
}

// GbClassicColors:
//     //White 202, 220, 159
//     //LiteG 139, 172, 15 
//     //DarkG 48,  98,  48 
//     //Black 15,  56,  15 

impl Pallets {
    pub fn init() -> Self {
        let mut pallets: HashMap<BulitinPallets, Pallet> = HashMap::new();

        let mut p = Pallet::new(4).ok().unwrap();
        p.set_color(0, Color{r: 202, g: 220, b:159});
        p.set_color(1, Color{r: 139, g: 172, b:15});
        p.set_color(2, Color{r: 48,  g: 98,  b:48});
        p.set_color(3, Color{r: 15,  g: 56,  b:15});
        pallets.insert(BulitinPallets::GbClassicColors, p);

        Pallets {pallets}
    }
}

impl Index<BulitinPallets> for Pallets {
    type Output = Pallet;
    
    fn index(&self, pallet: BulitinPallets) -> &Self::Output {
        &self.pallets[&pallet]
    }
}

#[derive(Debug)]
pub enum PalletErrors {
    InvalidNumberOfColors(usize),
}

impl Display for PalletErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PalletErrors::InvalidNumberOfColors(num_colors) => write!(f, "{} is not a valid number of colors. Please select 2 to 256 inclusive.", num_colors),
        }
    }
}

impl std::error::Error for PalletErrors { }