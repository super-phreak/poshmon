#![crate_name = "poshmon_nclient"]

pub mod graphics;

fn main() {
    use graphics::{Viewport, Pallets, BulitinPallets};
    let pallets = Pallets::init();
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        let viewport = Viewport::new(w-45, h, 0, 0, pallets[BulitinPallets::GbClassicColors].clone());
        viewport.print_pallet();
    } else {
        println!("Unable to get term size :(")
    }

    
}
