use nalgebra::SMatrix;

pub type Matrix4x4 = SMatrix<f64, 4, 4>;
pub type Matrix3x3 = SMatrix<f64, 3, 3>;
pub type Matrix2x2 = SMatrix<f64, 2, 2>;
pub type Matrix4x1 = SMatrix<f64, 4, 1>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::tuple::{eq_f64, point, Tuple, EPSILON};
    use crate::programs::arch::M;
    use nalgebra::Matrix;
    use std::ops::{Mul, Sub};

    fn sub_matrix_4x4(m: &Matrix4x4, row: usize, col: usize) -> Matrix3x3 {
        m.remove_row(row).remove_column(col)
    }

    fn sub_matrix_3x3(m: &Matrix3x3, row: usize, col: usize) -> Matrix2x2 {
        m.remove_row(row).remove_column(col)
    }

    fn minor(m: &Matrix4x4, row: usize, col: usize) -> f64 {
        sub_matrix_4x4(m, row, col).determinant()
    }

    fn is_invertible(m: &Matrix4x4) -> bool {
        !eq_f64(0.0, m.determinant())
    }

    fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;
        m
    }

    fn tup_into_4x1(t: Tuple) -> Matrix4x1 {
        let (a, b, c, d) = t;
        Matrix4x1::new(a, b, c, d)
    }

    fn m4x1_into_tup(m: &Matrix4x1) -> Tuple {
        let data = m.data.0[0];
        (data[0], data[1], data[2], data[3])
    }

    fn mul_4x4_and_4x1(m1: &Matrix4x4, m2: &Matrix4x1) -> Matrix4x1 {
        m1.mul(m2)
    }

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

    #[test]
    fn matrix_identity() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let m2 = m1.mul(Matrix4x4::identity());
        assert!(m1.relative_eq(&m2, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn matrix_transpose() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let m2 = m1.transpose();
        let mut expected = Matrix4x4::new(
            1.0, 2.0, 8.0, 0.0, 2.0, 4.0, 6.0, 0.0, 3.0, 4.0, 4.0, 0.0, 4.0, 2.0, 1.0, 1.0,
        );
        assert!(m2.relative_eq(&m2, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn matrix_determinant() {
        let mut m1 = Matrix2x2::new(1.0, 5.0, -3.0, 2.0);
        let det = m1.determinant();
        assert!(eq_f64(17.0, det));
    }

    #[test]
    fn matrix_resize_3x3() {
        let mut m1 = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let m2 = sub_matrix_3x3(&m1, 0, 0);
        let m3 = Matrix2x2::new(5.0, 6.0, 8.0, 9.0);
        assert!(m3.relative_eq(&m2, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn matrix_resize_4x4() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let m2 = sub_matrix_4x4(&m1, 0, 0);
        let m3 = Matrix3x3::new(6.0, 7.0, 8.0, 10.0, 11.0, 12.0, 14.0, 15.0, 16.0);
        assert!(m3.relative_eq(&m2, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn minor_4x4() {
        let mut m1 = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let m2 = sub_matrix_4x4(&m1, 0, 0);
        assert!(eq_f64(m2.determinant(), minor(&m1, 0, 0)));
    }

    #[test]
    fn determinant_4x4() {
        let mut m1 = Matrix4x4::new(
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        );
        assert!(eq_f64(-4071.0, m1.determinant()));
    }

    #[test]
    fn determinant_of_4x4_is_zero() {
        let mut m1 = Matrix4x4::new(
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        );
        assert!(eq_f64(0.0, m1.determinant()));
        assert!(!is_invertible(&m1));
    }

    #[test]
    fn determinant_of_4x4_is_not_zero() {
        let mut m1 = Matrix4x4::new(
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        );
        let a = -2120.0;
        let b = m1.determinant();
        assert!(eq_f64(a, b));
        assert!(is_invertible(&m1));
    }

    #[test]
    fn inverse_of_4x4() {
        let mut a = Matrix4x4::new(
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        );

        let b = a.try_inverse();
        assert!(b.is_some());
        let b = b.unwrap();
        assert!(eq_f64(b[(0, 0)], 0.21805));
        assert!(eq_f64(b[(1, 1)], -1.45677));
        assert!(eq_f64(b[(2, 0)], -0.07895));
        assert!(eq_f64(b[(0, 2)], 0.24060));
    }

    #[test]
    fn inverse_of_4x4_example_two() {
        // let mut a = Matrix4x4::new(8.0, -5.0);

        // let b = a.try_inverse();
        // assert!(b.is_some());
        // let b = b.unwrap();
        // assert!(eq_f64(b[(0, 0)], 0.21805));
        // assert!(eq_f64(b[(1, 1)], -1.45677));
        // assert!(eq_f64(b[(2, 0)], -0.07895));
        // assert!(eq_f64(b[(0, 2)], 0.24060));
    }

    #[test]
    fn inverse_of_4x4_example_three() {
        let mut a = Matrix4x4::new(
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        );

        let b = a.try_inverse();
        assert!(b.is_some());
        let b = b.unwrap();
        assert!(eq_f64(b[(0, 0)], -0.15385));
        let x = b[(1, 1)];
        assert!(eq_f64(x, 0.12308));
        let x = b[(2, 0)];
        assert!(eq_f64(x, 0.35897));
        let x = b[(0, 2)];
        assert!(eq_f64(x, -0.28205));
    }

    #[test]
    fn product_inverse() {
        let mut a = Matrix4x4::new(
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        );

        let mut b = Matrix4x4::new(
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        );

        let c = a.mul(b);

        let d = c.mul(b.try_inverse().unwrap());
        let rel = 0.000001;
        assert!(a.relative_eq(&d, rel, rel));
    }

    #[test]
    fn identity_inverse() {
        let a = Matrix4x4::identity();
        let b = a.try_inverse().unwrap();
        assert!(a.relative_eq(&b, f64::EPSILON, f64::EPSILON));
    }

    #[test]
    fn translation_one() {
        let m1 = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let m4x1 = tup_into_4x1(p);
        let multiplied = mul_4x4_and_4x1(&m1, &m4x1);
        let tup = m4x1_into_tup(&multiplied);
        assert_eq!(tup, (2.0, 1.0, 7.0, 1.0))
    }

    #[test]
    fn translation_two() {
        let m1 = translation(5.0, -3.0, 2.0);
        let m2 = m1.try_inverse().unwrap();
        let p = point(-3.0, 4.0, 5.0);
        let m4x1 = tup_into_4x1(p);
        let multiplied = mul_4x4_and_4x1(&m2, &m4x1);
        let tup = m4x1_into_tup(&multiplied);
        assert_eq!(tup, (-8.0, 7.0, 3.0, 1.0))
    }
}
