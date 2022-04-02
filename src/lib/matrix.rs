use nalgebra::SMatrix;

pub type Matrix4x4 = SMatrix<f64, 4, 4>;
pub type Matrix3x3 = SMatrix<f64, 3, 3>;
pub type Matrix2x2 = SMatrix<f64, 2, 2>;
pub type Matrix4x1 = SMatrix<f64, 4, 1>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::tuple::{eq_f64, Tuple};
    use std::ops::Mul;

    // pub fn mul_by_tup(mx: Matrix4x4, t: Tuple) -> Tuple {}

    #[test]
    fn matrix_constructs_properly_4x4() {
        let mut m = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );

        assert!(eq_f64(m[(0, 0)], 1.0));
        assert!(eq_f64(m[(0, 3)], 4.0));
        assert!(eq_f64(m[(1, 0)], 5.5));
        assert!(eq_f64(m[(1, 2)], 7.5));
        assert!(eq_f64(m[(2, 2)], 11.0));
        assert!(eq_f64(m[(3, 0)], 13.5));
        assert!(eq_f64(m[(3, 2)], 15.5));
    }

    #[test]
    fn matrix_constructs_properly_3x3() {
        let mut m = Matrix3x3::new(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);

        assert!(eq_f64(m[(0, 0)], -3.0));
        assert!(eq_f64(m[(1, 1)], -2.0));
        assert!(eq_f64(m[(2, 2)], 1.0));
        assert!(eq_f64(m[(2, 1)], 1.0));
    }

    #[test]
    fn matrix_constructs_properly_2x2() {
        let mut m = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);

        assert!(eq_f64(m[(0, 0)], -3.0));
        assert!(eq_f64(m[(0, 1)], 5.0));
        assert!(eq_f64(m[(1, 0)], 1.0));
        assert!(eq_f64(m[(1, 1)], -2.0));
    }

    #[test]
    fn matrices_are_equal() {
        let mut m1 = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);
        let mut m2 = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);
        assert!(m1.relative_eq(&m2, f64::EPSILON, f64::EPSILON))
    }

    #[test]
    fn matrices_are_not_equal() {
        let mut m1 = Matrix2x2::new(-2.95, 5.0, 1.0, -2.0);
        let mut m2 = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);
        assert!(!m1.relative_eq(&m2, f64::EPSILON, f64::EPSILON))
    }

    #[test]
    fn matrix_multiplication_4x4() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let mut m2 = Matrix4x4::new(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        );

        let m3 = m1.mul(m2);

        let mut expected = Matrix4x4::new(
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        );

        assert!(expected.relative_eq(&m3, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn matrix_multiplication_4x4_and_4x1() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );

        let mut m2 = Matrix4x1::new(1.0, 2.0, 3.0, 1.0);

        let m3 = m1.mul(m2);

        let mut expected = Matrix4x1::new(18.0, 24.0, 33.0, 1.0);

        assert!(expected.relative_eq(&m3, f64::EPSILON, f64::EPSILON));
    }
}
