#![allow(dead_code)]
use crate::canvas::Canvas;
use crate::color::Color;
use crate::raytuple::RayTuple;
use std::f64::consts::PI;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    size: i32,
    pub m: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new(size: i32) -> Self {
        Self {
            size,
            m: [[0.0; 4]; 4],
        }
    }

    pub fn new_matrix2(matrix: [[f64; 2]; 2]) -> Self {
        Self {
            size: 2,
            m: [
                [matrix[0][0], matrix[0][1], 0.0, 0.0],
                [matrix[1][0], matrix[1][1], 0.0, 0.0],
                [0.0; 4],
                [0.0; 4],
            ],
        }
    }

    pub fn new_matrix3(matrix: [[f64; 3]; 3]) -> Self {
        Self {
            size: 3,
            m: [
                [matrix[0][0], matrix[0][1], matrix[0][2], 0.0],
                [matrix[1][0], matrix[1][1], matrix[1][2], 0.0],
                [matrix[2][0], matrix[2][1], matrix[2][2], 0.0],
                [0.0; 4],
            ],
        }
    }

    pub fn new_matrix4(matrix: [[f64; 4]; 4]) -> Self {
        Self { size: 4, m: matrix }
    }

    pub fn identity() -> Self {
        Self {
            size: 4,
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn size(self) -> i32 {
        self.size
    }

    pub fn transpose(self) -> Self {
        Self {
            size: self.size,
            m: [
                [self[0][0], self[1][0], self[2][0], self[3][0]],
                [self[0][1], self[1][1], self[2][1], self[3][1]],
                [self[0][2], self[1][2], self[2][2], self[3][2]],
                [self[0][3], self[1][3], self[2][3], self[3][3]],
            ],
        }
    }

    //Recursively calculate the determinant of matrix regardless of size
    pub fn determinant(m: Matrix) -> f64 {
        let mut det: f64 = 0.0;

        if m.size() == 2 {
            det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
        } else {
            for col in 0_usize..m.size() as usize {
                det = det + m[0][col] * m.cofactor(0, col);
            }
        }

        det
    }

    //This will remove a row and column and reduce the Matrix dimensions
    pub fn submatrix(self, row: usize, col: usize) -> Matrix {
        let mut ret_matrix = Matrix::new(self.size - 1);

        let mut sh_i = 0;
        for i in 0_usize..3 {
            if i == row {
                sh_i = i + 1;
            }
            let mut sh_j = 0;
            for j in 0_usize..3 {
                if j == col {
                    sh_j = j + 1;
                }
                ret_matrix[i][j] = self[sh_i][sh_j];
                sh_j += 1;
            }
            sh_i += 1;
        }

        ret_matrix
    }

    //This uses submatrix and determinant
    pub fn minor(self, row: usize, col: usize) -> f64 {
        let b = self.clone().submatrix(row, col);
        Self::determinant(b)
    }

    pub fn cofactor(self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 1 {
            -self.minor(row, col)
        } else {
            self.minor(row, col)
        }
    }

    pub fn invertible(self) -> bool {
        Self::determinant(self) != 0.0
    }

    pub fn inverse(self) -> Option<Matrix> {
        if !self.invertible() {
            return None;
        }
        let mut m2 = Matrix::new(self.size);
        for row in 0_usize..m2.size as usize {
            for col in 0_usize..m2.size as usize {
                let c = self.cofactor(row, col);
                m2[col][row] = c / Matrix::determinant(self);
            }
        }
        Some(m2)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[0][3] = x;
        m[1][3] = y;
        m[2][3] = z;

        m
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;

        m
    }

    pub fn rotation_x(r: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[1][1] = r.cos();
        m[1][2] = -r.sin();
        m[2][1] = r.sin();
        m[2][2] = r.cos();

        m
    }

    pub fn rotation_y(r: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[0][0] = r.cos();
        m[0][2] = r.sin();
        m[2][0] = -r.sin();
        m[2][2] = r.cos();

        m
    }

    pub fn rotation_z(r: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[0][0] = r.cos();
        m[0][1] = -r.sin();
        m[1][0] = r.sin();
        m[1][1] = r.cos();

        m
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut m = Matrix::identity();
        m[0][1] = xy;
        m[0][2] = xz;
        m[1][0] = yx;
        m[1][2] = yz;
        m[2][0] = zx;
        m[2][1] = zy;

        m
    }

    pub fn view_transform(from: RayTuple, to: RayTuple, up: RayTuple) -> Matrix {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);

        let orientation = Matrix::new_matrix4([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let epsilon: f64 = 0.00001;

        f64::abs(self[0][0] - other[0][0]) < epsilon
            && f64::abs(self[0][1] - other[0][1]) < epsilon
            && f64::abs(self[0][2] - other[0][2]) < epsilon
            && f64::abs(self[0][3] - other[0][3]) < epsilon
            && f64::abs(self[1][0] - other[1][0]) < epsilon
            && f64::abs(self[1][1] - other[1][1]) < epsilon
            && f64::abs(self[1][2] - other[1][2]) < epsilon
            && f64::abs(self[1][3] - other[1][3]) < epsilon
            && f64::abs(self[2][0] - other[2][0]) < epsilon
            && f64::abs(self[2][1] - other[2][1]) < epsilon
            && f64::abs(self[2][2] - other[2][2]) < epsilon
            && f64::abs(self[2][3] - other[2][3]) < epsilon
            && f64::abs(self[3][0] - other[3][0]) < epsilon
            && f64::abs(self[3][1] - other[3][1]) < epsilon
            && f64::abs(self[3][2] - other[3][2]) < epsilon
            && f64::abs(self[3][3] - other[3][3]) < epsilon
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, i: usize) -> &[f64; 4] {
        &self.m[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut [f64; 4] {
        &mut self.m[i]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut ret_matrix = Matrix::new(self.size);

        for r in 0_usize..4 {
            for c in 0_usize..4 {
                ret_matrix[r][c] = self[r][0] * rhs[0][c]
                    + self[r][1] * rhs[1][c]
                    + self[r][2] * rhs[2][c]
                    + self[r][3] * rhs[3][c];
            }
        }

        ret_matrix
    }
}

impl Mul<RayTuple> for Matrix {
    type Output = RayTuple;

    fn mul(self, rhs: RayTuple) -> RayTuple {
        let mut ret_tuple = RayTuple::zero();

        ret_tuple.x =
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w;
        ret_tuple.y =
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w;
        ret_tuple.z =
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w;
        ret_tuple.w =
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w;

        ret_tuple
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn construct_matrix() {
        let m = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn construct_two_by_two() {
        let mut m = Matrix::new(2);
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn construct_three_by_three() {
        let mut m = Matrix::new(3);
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[0][2] = 0.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;
        m[1][2] = -7.0;
        m[2][0] = 0.0;
        m[2][1] = 1.0;
        m[2][2] = 1.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_equality() {
        let a = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(a == b);
    }

    #[test]
    fn matrix_inequality() {
        let a = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_matrix4([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(a != b);
    }

    #[test]
    fn matrix_multiply() {
        let a = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_matrix4([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            a * b,
            Matrix::new_matrix4([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn matrix_multiply_tuple() {
        let a = Matrix::new_matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = RayTuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(a * b, RayTuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiply_identity() {
        let a = Matrix::new_matrix4([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let b = Matrix::identity();
        assert_eq!(a * b, a);
    }

    #[test]
    fn transpose_test() {
        let a = Matrix::new_matrix4([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let at = Matrix::new_matrix4([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(a.transpose(), at);
    }

    #[test]
    fn transpose_identity() {
        let a = Matrix::identity();
        assert_eq!(a.transpose(), Matrix::identity());
    }

    #[test]
    fn determinant_of_two() {
        let mut a = Matrix::new(2);
        a[0][0] = 1.0;
        a[0][1] = 5.0;
        a[1][0] = -3.0;
        a[1][1] = 2.0;
        assert_eq!(Matrix::determinant(a), 17.0);
    }

    #[test]
    fn submatrix_of_three() {
        let a = Matrix::new_matrix3([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let b = Matrix::new_matrix2([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(a.submatrix(0, 2), b);
    }

    #[test]
    fn submatrix_of_four() {
        let a = Matrix::new_matrix4([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let b = Matrix::new_matrix3([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn minor_of_three() {
        let a = Matrix::new_matrix3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(1, 0), 25.0);
    }
    #[test]
    fn cofactor_test() {
        let a = Matrix::new_matrix3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let minor_a_0_0 = a.minor(0, 0);
        let cofactor_a_0_0 = a.cofactor(0, 0);
        let minor_a_1_0 = a.minor(1, 0);
        let cofactor_a_1_0 = a.cofactor(1, 0);
        assert_eq!(minor_a_0_0, -12.0);
        assert_eq!(cofactor_a_0_0, -12.0);
        assert_eq!(minor_a_1_0, 25.0);
        assert_eq!(cofactor_a_1_0, -25.0);
    }

    #[test]
    fn determinant_of_three() {
        let a = Matrix::new_matrix3([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(Matrix::determinant(a), -196.0);
    }

    #[test]
    fn determinant_of_four() {
        let a = Matrix::new_matrix4([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(Matrix::determinant(a), -4071.0);
    }

    #[test]
    fn is_invertible() {
        let a = Matrix::new_matrix4([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(Matrix::determinant(a), -2120.0);
        assert!(a.invertible());
    }

    #[test]
    fn not_invertible() {
        let a = Matrix::new_matrix4([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(Matrix::determinant(a), 0.0);
        assert!(!a.invertible());
    }

    #[test]
    fn inverse_of_matrix() {
        let a = Matrix::new_matrix4([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse().unwrap();
        assert_eq!(Matrix::determinant(a), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[3][2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[2][3], 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix::new_matrix4([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn inverse_two() {
        let a = Matrix::new_matrix4([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        assert_eq!(
            a.inverse().unwrap(),
            Matrix::new_matrix4([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn inverse_three() {
        let a = Matrix::new_matrix4([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        assert_eq!(
            a.inverse().unwrap(),
            Matrix::new_matrix4([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn multiply_inverse() {
        let a = Matrix::new_matrix4([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new_matrix4([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[test]
    fn multiply_point_by_translation() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = RayTuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, RayTuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_point_by_inverse_translation() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = RayTuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, RayTuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translated_vector_doesnt_change() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = RayTuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = RayTuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, RayTuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = RayTuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, RayTuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn inverse_scaling_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = RayTuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, RayTuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_negative_scaling() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotate_point_around_x() {
        let p = RayTuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            RayTuple::point(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, RayTuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_rotate_point_around_x() {
        let p = RayTuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            RayTuple::point(0.0, (2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0)
        );
    }

    #[test]
    fn rotate_point_around_y() {
        let p = RayTuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            RayTuple::point((2.0_f64).sqrt() / 2.0, 0.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, RayTuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotate_point_around_z() {
        let p = RayTuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            RayTuple::point(-(2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, RayTuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear_x_in_proportion_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shear_x_in_proportion_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shear_y_in_proportion_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shear_y_in_proportion_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shear_z_in_proportion_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shear_z_in_proportion_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = RayTuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, RayTuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn transformations_in_sequence() {
        let p = RayTuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, RayTuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, RayTuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, RayTuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformations_at_once() {
        let p = RayTuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        assert_eq!(t * p, RayTuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transform_default() {
        let from = RayTuple::point(0.0, 0.0, 0.0);
        let to = RayTuple::point(0.0, 0.0, -1.0);
        let up = RayTuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn view_transform_positive_z() {
        let from = RayTuple::point(0.0, 0.0, 0.0);
        let to = RayTuple::point(0.0, 0.0, 1.0);
        let up = RayTuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_translation_moves_the_world() {
        let from = RayTuple::point(0.0, 0.0, 8.0);
        let to = RayTuple::point(0.0, 0.0, 0.0);
        let up = RayTuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn view_translation_arbitrary() {
        let from = RayTuple::point(1.0, 3.0, 2.0);
        let to = RayTuple::point(4.0, -2.0, 8.0);
        let up = RayTuple::vector(1.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        let m = Matrix::new_matrix4([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        assert_eq!(t, m);
    }
}

pub fn chapter_three_matrix() {
    //what happens when you invert the identity matrix?
    let identity = Matrix::identity();
    println!("Inverted identity matrix: {:?}", identity.inverse());
    //A: It doesn't change

    //what do you get when you multiply a matrix by it's inverse?
    let a = Matrix::new_matrix4([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, -8.0, 7.0, -6.0],
        [5.0, 4.0, -3.0, 2.0],
    ]);
    let b = a.inverse().unwrap();
    let c = a * b;
    println!("A multiplied by it's inverse, a * a^-1 = {:?}", c);
    //A: When you multiply a matrix by it's inverse you get the identity matrix

    //Is there any difference between the inverse of the transpose, and the transpose of the inverse of a matrix?
    let a = Matrix::new_matrix4([
        [-2.0, 5.0, 0.0, 6.0],
        [6.0, -4.0, -3.0, -3.0],
        [17.0, 1.0, -2.0, 8.0],
        [0.0, 0.0, 3.0, -4.0],
    ]);
    let it = a.inverse().unwrap().transpose();
    let ti = a.transpose().inverse().unwrap();

    println!("There is no difference between the inverse of the transpose and the transpose of the inverse: {}", it==ti);

    //Given multiplying the identity matrix by a tuple gives the tuple unchanged, what happens when you change
    //any single element of the identity matrix prior to multiplying?
    let r = RayTuple::new(1.0, 1.0, 1.0, 1.0);
    let mut i = Matrix::identity();

    assert_eq!(RayTuple::new(1.0, 1.0, 1.0, 1.0), i * r);

    i[1][3] = 2.0;
    let ir = i * r;

    println!("i * r = {:?}", ir);
    //A: It appears that altering other elements in the identity matrix adds to the tuple in some way
    //this is likely useful for translation? changing the second row, fourth column to 2.0 ended up adding 2
    //to the y coordinate in the tuple.
}

pub fn chapter_four_clockpoints() {
    let mut can = Canvas::new(800, 800);
    let radial_interval = PI / 6.0;
    let clock_radius: f64 = (can.get_width() / 3) as f64;
    let x_offset = (can.get_width() / 2) as f64;
    let y_offset = (can.get_height() / 2) as f64;
    let plot_color = Color::new(1.0, 1.0, 1.0);

    let mut rotation_angle = 0.0;
    let mut plot_point = RayTuple::point(0.0, clock_radius, 0.0);
    while rotation_angle <= (2.0 * PI) {
        let x: i32 = (x_offset + plot_point.x).round() as i32;
        let y: i32 = (y_offset + plot_point.y).round() as i32;
        can.write_pixel(x, y, plot_color);
        println!(
            "Rotation angle: {}, plotting point {},{}",
            rotation_angle, plot_point.x, plot_point.y
        );

        rotation_angle += radial_interval;
        let t = Matrix::rotation_z(radial_interval);
        plot_point = t * plot_point;
    }
    can.save_ppm("clockface.ppm");
}
