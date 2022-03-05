use std::ops::Neg;

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
/// Determines equality between two 64-bit floating-point values
///
pub fn eq_f64(left: f64, right: f64) -> bool {
    (left - right).abs() < EPSILON
}

///
/// Determines equality between two tuples
///
pub fn eq_tup(left: (f64, f64, f64, f64), right: (f64, f64, f64, f64)) -> bool {
    eq_f64(left.0, right.0) &&
        eq_f64(left.1, right.1) &&
        eq_f64(left.2, right.2) &&
        eq_f64(left.3, right.3)
}

///
/// Adds two tuples together
///
pub fn add_tup(left: (f64, f64, f64, f64), right: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    if is_point(left) && is_point(right) {
        panic!("adding two points!")
    }
    (left.0 + right.0, left.1 + right.1, left.2 + right.2, left.3 + right.3)
}

///
/// Subtracts the right tuple from the left tuple
///
pub fn sub_tup(left: (f64, f64, f64, f64), right: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    (left.0 - right.0, left.1 - right.1, left.2 - right.2, left.3 - right.3)
}

///
/// Subtracts the right tuple from the left tuple
///
pub fn neg_tup(tup: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    if is_point(tup) {
        panic!("attempting to negate a point")
    }

    (tup.0.neg(), tup.1.neg(), tup.2.neg(), tup.3.neg())
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

    #[test]
    fn add_vector_and_point() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let pt = point(x1, y1, z1);
        let vec = vector(x2, y2, z2);
        let new_pt = add_tup(pt, vec);

        assert!(is_point(new_pt));
        assert!(eq_f64(x1 + x2, new_pt.0));
        assert!(eq_f64(y1 + y2, new_pt.1));
        assert!(eq_f64(z1 + z2, new_pt.2));
        assert!(eq_f64(POINT_INDICATOR, new_pt.3));
    }

    #[test]
    fn add_vector_and_vector() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let v1 = vector(x1, y1, z1);
        let v2 = vector(x2, y2, z2);
        let v3 = add_tup(v1, v2);

        assert!(is_vector(v3));
        assert!(eq_f64(x1 + x2, v3.0));
        assert!(eq_f64(y1 + y2, v3.1));
        assert!(eq_f64(z1 + z2, v3.2));
        assert!(eq_f64(VECTOR_INDICATOR, v3.3));
    }

    #[test]
    #[should_panic]
    fn add_point_and_point() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let p1 = point(x1, y1, z1);
        let p2 = point(x2, y2, z2);
        let new_pt = add_tup(p1, p2);

        assert!(is_point(new_pt));
        assert!(eq_f64(x1 + x2, new_pt.0));
        assert!(eq_f64(y1 + y2, new_pt.1));
        assert!(eq_f64(z1 + z2, new_pt.2));
        assert!(eq_f64(POINT_INDICATOR, new_pt.3));
    }

    #[test]
    fn sub_point_and_point() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let p1 = point(x1, y1, z1);
        let p2 = point(x2, y2, z2);
        let vec = sub_tup(p1, p2);

        assert!(is_vector(vec));
        assert!(eq_f64(x1 - x2, vec.0));
        assert!(eq_f64(y1 - y2, vec.1));
        assert!(eq_f64(z1 - z2, vec.2));
        assert!(eq_f64(VECTOR_INDICATOR, vec.3));
    }

    #[test]
    fn sub_point_and_vector() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let pt = point(x1, y1, z1);
        let vec = vector(x2, y2, z2);
        let new_pt = sub_tup(pt, vec);

        assert!(is_point(new_pt));
        assert!(eq_f64(x1 - x2, new_pt.0));
        assert!(eq_f64(y1 - y2, new_pt.1));
        assert!(eq_f64(z1 - z2, new_pt.2));
        assert!(eq_f64(POINT_INDICATOR, new_pt.3));
    }

    #[test]
    fn sub_vector_and_vector() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (10.0, 20.0, 30.0);

        let v1 = vector(x1, y1, z1);
        let v2 = vector(x2, y2, z2);
        let v3 = sub_tup(v1, v2);

        assert!(is_vector(v3));
        assert!(eq_f64(x1 - x2, v3.0));
        assert!(eq_f64(y1 - y2, v3.1));
        assert!(eq_f64(z1 - z2, v3.2));
        assert!(eq_f64(VECTOR_INDICATOR, v3.3));
    }

    #[test]
    fn negate_a_vector() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);

        let vec = neg_tup(vector(x1, y1, z1));

        assert!(is_vector(vec));
        assert!(eq_f64(x1.neg(), vec.0));
        assert!(eq_f64(y1.neg(), vec.1));
        assert!(eq_f64(z1.neg(), vec.2));
        assert!(eq_f64(VECTOR_INDICATOR, vec.3));
    }

    #[test]
    #[should_panic]
    fn negate_a_point() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);

        let vec = neg_tup(point(x1, y1, z1));

        assert!(is_vector(vec));
        assert!(eq_f64(x1.neg(), vec.0));
        assert!(eq_f64(y1.neg(), vec.1));
        assert!(eq_f64(z1.neg(), vec.2));
        assert!(eq_f64(POINT_INDICATOR, vec.3));
    }
}