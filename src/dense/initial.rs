use crate::dense::{error, Matrix};
use rand::Rng;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use array_init::array_init;

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
        let data = array_init(|_| v);
        Matrix { data, digits: 0 }
    }

    // 生成单位方阵
    pub fn unit() -> Self {
        let data = array_init(|i| {
            let row = i / COLS;
            let col = i % COLS;
            if row == col && row < ROWS.min(COLS) {
                T::from(1)
            } else {
                T::default()
            }
        });
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

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS, f64> {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let data = array_init(|_| rng.random());
        Matrix { data, digits: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_with_default() {
        let m: Matrix<3, 3, f64> = Matrix::new_with_default(1.2);
        assert_eq!(m.size(), (3, 3));
        for row in 0..3 {
            for col in 0..3 {
                let v = m.get(row, col).unwrap();
                assert_eq!(1.2, v)
            }
        }
    }
    
    #[test]
    fn test_unit_matrix() {
        let m: Matrix<3, 3, f64> = Matrix::unit();
        println!("{}", m);
        assert_eq!(m.data, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_zeros() {
        let m: Matrix<3, 3, f64> = Matrix::zeros();
        assert_eq!(m.data, [0.0; 9]);
        assert_eq!(m.size(), (3, 3));

        let m: Matrix<4, 4, f64> = Matrix::ones();
        assert_eq!(m.data, [1.0; 16]);
        assert_eq!(m.size(), (4, 4));
    }

    #[test]
    fn test_ones() {
        let m: Matrix<4, 3, f64> = Matrix::ones();
        assert_eq!(m.data, [1.0; 12]);
        assert_eq!(m.size(), (4, 3));
    }

    #[test]
    fn test_random() {
        let m: Matrix<3, 3, f64> = Matrix::random();
        println!("{}", m);
        assert_eq!(m.size(), (3, 3));
    }
}
