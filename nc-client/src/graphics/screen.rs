use std::{error::Error, io::Stdout};

use super::{Canvas, Viewport};

pub trait Screen {
    fn new (cavas: Canvas, viewport: Viewport) -> Self;

    fn render(&self, stdout: &mut Stdout, delta_time: f64) -> Result<(), Box<dyn Error>>;

    fn update(&self, delta_time: f64) -> Result<(), Box<dyn Error>>;
}