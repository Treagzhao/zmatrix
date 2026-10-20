use crate::dense::{Matrix, NumericElement};
use rand::Rng;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: NumericElement
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Display
        + Default
        + From<i8>,
{
    pub fn new_with_default(v: T) -> Self {
        const { assert!(ROWS > 0 && COLS > 0, "Matrix 的行数和列数都必须大于 0"); }
        let data:[[T;COLS];ROWS] = [[v;COLS];ROWS];
        Matrix { data }
    }

    // 生成全零矩阵
    pub fn zeros() -> Self {
        Self::new_with_default(T::default())
    }

    // 生成全1矩阵
    pub fn ones() -> Self {
        Self::new_with_default(T::from(1))
    }
}

impl<const N: usize, T> Matrix<N, N, T>
where
    T: NumericElement + From<i8> + Default,
{
    // 生成单位方阵
    pub fn unit() -> Self {
        const { assert!(N > 0, "Matrix 的行数和列数都必须大于 0"); }
        let mut data = [[T::default(); N]; N];
        for i in 0..N {
            data[i][i] = T::from(1);
        }
        Matrix { data }
    }
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS, f64> {
    pub fn random() -> Self {
        const { assert!(ROWS > 0 && COLS > 0, "Matrix 的行数和列数都必须大于 0"); }
        let mut rng = rand::rng();
        let mut data = [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = rng.random();
            }
        }
        Matrix { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let m: Matrix<3, 3, f64> = Matrix::zeros();
        assert_eq!(m.data, [[0.0; 3]; 3]);
        assert_eq!(m.size(), (3, 3));
    }

    #[test]
    fn test_unit_matrix() {
        let m: Matrix<3, 3, f64> = Matrix::unit();
        println!("{}", m);
        assert_eq!(m.data, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    }

    #[test]
    fn test_ones() {
        let m1: Matrix<4, 4, f64> = Matrix::ones();
        assert_eq!(m1.data, [[1.0; 4]; 4]);
        assert_eq!(m1.size(), (4, 4));

        let m2: Matrix<4, 3, f64> = Matrix::ones();
        assert_eq!(m2.data, [[1.0; 3]; 4]);
        assert_eq!(m2.size(), (4, 3));
    }

    #[test]
    fn test_random() {
        let m: Matrix<3, 3, f64> = Matrix::random();
        println!("{}", m);
        assert_eq!(m.size(), (3, 3));
    }
}
