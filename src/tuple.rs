pub static EPSILON: f64 = 0.00001;
pub static POINT_INDICATOR: f64 = 1.0;
pub static VECTOR_INDICATOR: f64 = 0.0;

///
/// Creates a tuple representing a point
///
pub fn point(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    (x, y, z, POINT_INDICATOR)
}

///
/// Creates a tuple representing a vector
///
pub fn vector(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    (x, y, z, VECTOR_INDICATOR)
}

///
/// Determines if this tuple is a vector
///
pub fn is_vector(tuple: (f64, f64, f64, f64)) -> bool {
    eq_f64(tuple.3, VECTOR_INDICATOR)
}

///
/// Determines if this tuple is a point
///
pub fn is_point(tuple: (f64, f64, f64, f64)) -> bool {
    eq_f64(tuple.3, POINT_INDICATOR)
}

///
/// Determines equality between two tuples
///
pub fn eq_tup(t1: (f64, f64, f64, f64), t2: (f64, f64, f64, f64)) -> bool {
    eq_f64(t1.0, t2.0) &&
        eq_f64(t1.1, t2.1) &&
        eq_f64(t1.2, t2.2) &&
        eq_f64(t1.3, t2.3)
}

///
/// Determines equality between two 64-bit floating-point values
///
pub fn eq_f64(left: f64, right: f64) -> bool {
    (left - right).abs() < EPSILON
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_point() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt = point(x, y, z);

        assert!(is_point(pt))
    }

    #[test]
    fn point_is_not_vector() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt = point(x, y, z);

        assert!(!is_vector(pt))
    }

    #[test]
    fn vector_is_vector() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let vec = vector(x, y, z);

        assert!(is_vector(vec))
    }

    #[test]
    fn vector_is_not_point() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let vec = vector(x, y, z);

        assert!(!is_point(vec))
    }

    #[test]
    fn point_is_not_equal_to_vector() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt = point(x, y, z);
        let vec = vector(x, y, z);

        assert!(!eq_tup(pt, vec))
    }

    #[test]
    fn two_points_match() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let pt1 = point(x, y, z);
        let pt2 = point(x, y, z);

        assert!(eq_tup(pt1, pt2))
    }

    #[test]
    fn two_vectors_match() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let v1 = vector(x, y, z);
        let v2 = vector(x, y, z);

        assert!(eq_tup(v1, v2))
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

    #[test]
    fn f64_is_eq() {
        let (x, y) = (1.0, 1.000001);
        assert!(eq_f64(x, y))
    }

    #[test]
    fn f64_is_ne() {
        let (x, y) = (1.0, 1.1);
        assert!(!eq_f64(x, y))
    }
}