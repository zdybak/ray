#![allow(dead_code)]
use std::ops::{Index, IndexMut};

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
}
