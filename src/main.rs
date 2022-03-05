use image::{ImageBuffer, ImageFormat, Rgb};
use std::path::Path;

mod color;
mod tuple;

fn main() {
    let w = 50;
    let h = 50;
    let mut canvas: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    canvas
        .save_with_format(&Path::new("canvas.ppm"), ImageFormat::Pnm)
        .unwrap();
}
