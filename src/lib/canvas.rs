use image::{ImageBuffer, Rgb};

pub fn new_image_buffer(w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::new(w, h)
}
