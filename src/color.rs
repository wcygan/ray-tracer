pub type Color = (f64, f64, f64);

///
/// Creates a tuple representing a color
///
pub fn color(r: f64, g: f64, b: f64) -> Color {
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::eq_f64;

    #[test]
    fn make_color() {
        let (r, g, b) = (-0.5, 0.4, 1.7);
        let c = color(r, g, b);
        assert!(eq_f64(r, c.0));
        assert!(eq_f64(g, c.1));
        assert!(eq_f64(b, c.2));
    }
}
