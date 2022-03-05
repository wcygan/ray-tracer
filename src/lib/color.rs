use image::Rgb;
use std::ops::Mul;

pub const RBG_MAX_INT: u8 = 255;
pub const MAX_COLOR: f64 = 1.0;
pub const MIN_COLOR: f64 = 0.0;

pub type Color = (f64, f64, f64);
pub type Pixel = [u8; 3];

///
/// Creates a tuple representing a Color
///
pub fn color(r: f64, g: f64, b: f64) -> Color {
    (r, g, b)
}

///
/// Adds two Colors together
///
pub fn add_colors(left: Color, right: Color) -> Color {
    (left.0 + right.0, left.1 + right.1, left.2 + right.2)
}

///
/// Subtracts the right Color from the left Color
///
pub fn subtract_colors(left: Color, right: Color) -> Color {
    (left.0 - right.0, left.1 - right.1, left.2 - right.2)
}

///
/// Multiplies a tuple by a scalar
///
pub fn multiply_by_scalar(tup: Color, scalar: f64) -> Color {
    (tup.0.mul(scalar), tup.1.mul(scalar), tup.2.mul(scalar))
}

///
/// Multiplies a tuple by a scalar
///
pub fn multiply_by_color(left: Color, right: Color) -> Color {
    (left.0 * right.0, left.1 * right.1, left.2 * right.2)
}

///
/// Creates a Pixel from a Color
///
pub fn color_into_pixel(c: Color) -> Pixel {
    [c.0, c.1, c.2].map(|v| {
        if v >= MAX_COLOR {
            RBG_MAX_INT
        } else if v <= 0.0 {
            0
        } else {
            (RBG_MAX_INT as f64 * v).round() as u8
        }
    })
}

///
/// Creates an Rgb<u8> from a Pixel
///
pub fn pixel_into_rgb(p: Pixel) -> Rgb<u8> {
    Rgb(p)
}

///
/// Creates a Rgb<u8> from a Color
///
pub fn color_into_rgb(c: Color) -> Rgb<u8> {
    pixel_into_rgb(color_into_pixel(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::tuple::*;

    #[test]
    fn make_color() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c = color(r, g, b);
        assert!(eq_f64(r, c.0));
        assert!(eq_f64(g, c.1));
        assert!(eq_f64(b, c.2));
    }

    #[test]
    fn add_two_colors() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c1 = color(r, g, b);

        let (r, g, b) = (-1.0, 0.6, 0.3);
        let c2 = color(r, g, b);

        let c3 = add_colors(c1, c2);
        assert!(eq_f64(-1.5, c3.0));
        assert!(eq_f64(1.0, c3.1));
        assert!(eq_f64(2.0, c3.2));
    }

    #[test]
    fn sub_two_colors() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c1 = color(r, g, b);

        let (r, g, b) = (-1.0, 0.6, 0.3);
        let c2 = color(r, g, b);

        let c3 = subtract_colors(c1, c2);
        assert!(eq_f64(0.5, c3.0));
        assert!(eq_f64(-0.2, c3.1));
        assert!(eq_f64(1.4, c3.2));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c1 = color(r, g, b);
        let scalar = 2.0;

        let c3 = multiply_by_scalar(c1, scalar);
        assert!(eq_f64(-1.0, c3.0));
        assert!(eq_f64(0.8, c3.1));
        assert!(eq_f64(3.4, c3.2));
    }

    #[test]
    fn multiply_color_by_color() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c1 = color(r, g, b);

        let (r, g, b) = (1.0, 0.01, 2.0);
        let c2 = color(r, g, b);

        let c3 = multiply_by_color(c1, c2);
        assert!(eq_f64(-0.5, c3.0));
        assert!(eq_f64(0.004, c3.1));
        assert!(eq_f64(3.4, c3.2));
    }

    #[test]
    fn test_color_into_pixel_one() {
        let (r, g, b) = (0.0, 0.0, 0.0);
        let c = color(r, g, b);
        let p = color_into_pixel(c);

        assert_eq!(*p.get(0).unwrap(), 0);
        assert_eq!(*p.get(1).unwrap(), 0);
        assert_eq!(*p.get(2).unwrap(), 0);
    }

    #[test]
    fn test_color_into_pixel_two() {
        let (r, g, b) = (0.5452, 0.2, 0.1);
        let c = color(r, g, b);
        let p = color_into_pixel(c);

        assert_eq!(*p.get(0).unwrap(), 139);
        assert_eq!(*p.get(1).unwrap(), 51);
        assert_eq!(*p.get(2).unwrap(), 26);
    }

    #[test]
    fn test_color_into_rgb_one() {
        let (r, g, b) = (0.0, 0.0, 0.0);
        let c = color(r, g, b);
        let p = color_into_rgb(c);

        assert_eq!(*p.0.get(0).unwrap(), 0);
        assert_eq!(*p.0.get(1).unwrap(), 0);
        assert_eq!(*p.0.get(2).unwrap(), 0);
    }

    #[test]
    fn test_color_into_rgb_two() {
        let (r, g, b) = (0.5452, 0.2, 0.1);
        let c = color(r, g, b);
        let p = color_into_rgb(c);

        assert_eq!(*p.0.get(0).unwrap(), 139);
        assert_eq!(*p.0.get(1).unwrap(), 51);
        assert_eq!(*p.0.get(2).unwrap(), 26);
    }
}
