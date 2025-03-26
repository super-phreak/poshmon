pub mod graphics;

fn main() {
    use graphics::gameboy::{Viewport, Pallet};
    
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        let viewport = Viewport::new(w-45, h, 0, 0, Pallet::GB_CLASSIC_PIXELS);
        viewport.print_pallet();
    } else {
        println!("Unable to get term size :(")
    }

    
}
