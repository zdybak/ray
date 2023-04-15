#![allow(dead_code)]
use crate::raytuple::RayTuple;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub m: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub fn new_with_matrix(matrix: [[f64; 4]; 4]) -> Self {
        Self { m: matrix }
    }

    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            m: [
                [self[0][0], self[1][0], self[2][0], self[3][0]],
                [self[0][1], self[1][1], self[2][1], self[3][1]],
                [self[0][2], self[1][2], self[2][2], self[3][2]],
                [self[0][3], self[1][3], self[2][3], self[3][3]],
            ],
        }
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
        let mut ret_matrix = Matrix::new();

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

    #[test]
    fn construct_matrix() {
        let m = Matrix::new_with_matrix([
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
        let mut m = Matrix::new();
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
        let m = Matrix::new_with_matrix([
            [-3.0, 5.0, 0.0, 0.0],
            [1.0, -2.0, -7.0, 0.0],
            [0.0, 1.0, 1.0, 0.0],
            [0.0; 4],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_equality() {
        let a = Matrix::new_with_matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_with_matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(a == b);
    }

    #[test]
    fn matrix_inequality() {
        let a = Matrix::new_with_matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_with_matrix([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(a != b);
    }

    #[test]
    fn matrix_multiply() {
        let a = Matrix::new_with_matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_with_matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            a * b,
            Matrix::new_with_matrix([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn matrix_multiply_tuple() {
        let a = Matrix::new_with_matrix([
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
        let a = Matrix::new_with_matrix([
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
        let a = Matrix::new_with_matrix([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let at = Matrix::new_with_matrix([
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
}
