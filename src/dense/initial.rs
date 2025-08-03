use crate::dense::{error, Matrix};
use array_init::array_init;
use rand::Rng;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Display
        + Default
        + Send
        + Sync
        + TryInto<f64>
        + From<i8>,
    f64: From<T>,
{
    pub fn new_with_default(v: T) -> Self {
        let data:[[T;COLS];ROWS] = [[v;COLS];ROWS];
        Matrix { data, digits: 0 }
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
    T: Copy + From<i8> + Default + Send + Sync,
{
    // 生成单位方阵
    pub fn unit() -> Self {
        let mut data = [[T::default(); N]; N];
        for i in 0..N {
            data[i][i] = T::from(1);
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS, f64> {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let mut data = [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = rng.random();
            }
        }
        Matrix { data, digits: 0 }
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
