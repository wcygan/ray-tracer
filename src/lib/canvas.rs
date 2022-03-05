use image::{ImageBuffer, Rgb};

pub fn new_image_buffer(w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::new(w, h)
}

///
/// Determines if the point (x, y) resides in the rectangle from point (0..width, 0..height)
///
pub fn point_is_in_rectangle(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

///
/// image.rs library uses top-down coords, so we switch them to bottom-up here
///
pub fn convert_to_bottom_up_coordinates(x: i32, y: i32, _w: i32, h: i32) -> (i32, i32) {
    (x, h - y)
}

///
/// picks all neighbors within "n" distance from (x, y) that are valid points
///
pub fn neighbors(x: i32, y: i32, w: i32, h: i32, n: i32) -> Vec<(i32, i32)> {
    let mut v = vec![];
    for i in -n..n {
        for j in -n..n {
            let (mx, my) = (x + i, y + j);
            if point_is_in_rectangle(mx, my, w, h) {
                v.push((mx, my))
            }
        }
    }
    v
}
