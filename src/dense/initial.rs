use crate::dense::{error, Matrix};
use rand::Rng;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

impl<T> Matrix<T>
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
    pub fn new_with_default(
        height: u64,
        width: u64,
        v: T,
    ) -> Result<Matrix<T>, error::OperationError>
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
        let vec: Vec<T> = vec![v; (height * width) as usize];
        Matrix::new(height, width, vec)
    }

    // 生成单位方阵
    pub fn unit(height: u64) -> Result<Matrix<T>, error::OperationError> {
        let mut vec = vec![T::default(); (height * height) as usize];
        for i in 0..height {
            let index = i * height + i;
            vec[index as usize] = T::from(1);
        }
        Matrix::new(height, height, vec)
    }
    // 生成全零矩阵
    pub fn zeros(height: u64, width: u64) -> Result<Matrix<T>, error::OperationError> {
        Matrix::new_with_default(height, width, T::default())
    }
    // 生成全1矩阵
    pub fn ones(height: u64, width: u64) -> Result<Matrix<T>, error::OperationError> {
        Matrix::new_with_default(height, width, T::from(1))
    }

    //生成0~1的随机均匀矩阵
}

impl Matrix<f64> {
    pub fn random(height: u64, width: u64) -> Result<Matrix<f64>, error::OperationError> {
        let mut vec = vec![0.0; (height * width) as usize];
        let mut rng = rand::rng();
        for num in &mut vec {
            *num = rng.random();
        }
        Matrix::new(height, width, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_with_default() {
        let m: Matrix<f64> = Matrix::new_with_default(3, 3, 1.2).unwrap();
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
        let m: Matrix<f64> = Matrix::unit(3).unwrap();
        println!("{}", m);
        assert_eq!(vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], m.data);
    }

    #[test]
    fn test_zeros() {
        let m: Matrix<f64> = Matrix::zeros(3, 3).unwrap();
        assert_eq!(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], m.data);
        assert_eq!(m.size(), (3, 3));

        let m: Matrix<f64> = Matrix::ones(4, 4).unwrap();
        assert_eq!(
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            m.data
        );
        assert_eq!(m.size(), (4, 4));
    }

    #[test]
    fn test_ones() {
        let m: Matrix<f64> = Matrix::ones(4, 3).unwrap();
        assert_eq!(
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            m.data
        );
        assert_eq!(m.size(), (4, 3));
    }

    #[test]
    fn test_random() {
        let m: Matrix<f64> = Matrix::random(3, 3).unwrap();
        println!("{}", m);
        assert_eq!(m.size(), (3, 3));
    }
}
