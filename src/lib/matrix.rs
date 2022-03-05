use nalgebra::SMatrix;

pub type Matrix4x4 = SMatrix<f64, 4, 4>;
pub type Matrix3x3 = SMatrix<f64, 3, 3>;
pub type Matrix2x2 = SMatrix<f64, 2, 2>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::tuple::eq_f64;

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
}
