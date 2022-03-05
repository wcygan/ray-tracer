pub static POINT_INDICATOR: f64 = 1.0;
pub static VECTOR_INDICATOR: f64 = 0.0;


pub fn point(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    (x, y, z, POINT_INDICATOR)
}

pub fn vector(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    (x, y, z, VECTOR_INDICATOR)
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, POINT_INDICATOR, vector, VECTOR_INDICATOR};

    #[test]
    fn point_is_not_a_tuple() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt = point(x, y, z);
        let vec = vector(x, y, z);

        assert_ne!(pt, vec)
    }

    #[test]
    fn two_points_match() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt1 = point(x, y, z);
        let pt2 = point(x, y, z);

        assert_eq!(pt1, pt2)
    }

    #[test]
    fn two_vectors_match() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let v1 = vector(x, y, z);
        let v2 = vector(x, y, z);

        assert_eq!(v1, v2)
    }

    #[test]
    fn vector_values() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let vec = vector(x, y, z);

        assert_eq!(x, vec.0);
        assert_eq!(y, vec.1);
        assert_eq!(z, vec.2);
        assert_eq!(VECTOR_INDICATOR, vec.3);
    }

    #[test]
    fn point_values() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt = point(x, y, z);

        assert_eq!(x, pt.0);
        assert_eq!(y, pt.1);
        assert_eq!(z, pt.2);
        assert_eq!(POINT_INDICATOR, pt.3);
    }
}