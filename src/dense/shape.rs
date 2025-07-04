use crate::dense::error::OperationError;
use crate::dense::Matrix;
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
    pub fn T(&self) -> Matrix<T> {
        let mut vec: Vec<T> = vec![T::default(); (self.height * self.width) as usize];
        for row in 0..self.height {
            for col in 0..self.width {
                let index: usize = (row * self.width + col) as usize;
                let target_index: usize = (col * self.height + row) as usize;
                vec[target_index] = self.data[index];
            }
        }
        //这里宽和高参数换位置了
        let m = Matrix::new(self.width, self.height, vec).unwrap();
        m
    }

    pub fn reshape(&self, height: usize, width: usize) -> Result<Matrix<T>, OperationError> {
        if self.height * self.width != height * width {
            return Err(OperationError {
                message: "shape size does not match".to_string(),
            });
        }
        let data = self.data.clone();
        Matrix::new(height, width, data)
    }
    // 矩阵的水平拼接
    pub fn hstack(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, OperationError> {
        if self.height != rhs.height {
            return Err(OperationError {
                message: "shape size does not match".to_string(),
            });
        }
        let mut data: Vec<T> = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                data.push(self.data[(i * self.width + j) as usize]);
            }
            for j in 0..rhs.width {
                data.push(rhs.data[(i * rhs.width + j) as usize]);
            }
        }
        Matrix::new(self.height, self.width + rhs.width, data)
    }

    //矩阵的垂直拼接
    pub fn vstack(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, OperationError> {
        if self.width != rhs.width {
            return Err(OperationError {
                message: "shape size does not match".to_string(),
            });
        }
        let mut data: Vec<T> = self.data.clone();
        data.extend(rhs.data.clone());
        Matrix::new(self.height + rhs.height, self.width, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_T() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = m.T();
        assert_eq!(3, m1.height);
        assert_eq!(2, m1.width);
        println!("{}", m1);
        assert_eq!(vec![1, 4, 2, 5, 3, 6], m1.data);
        let vec: Vec<i32> = vec![];
        let m = Matrix::new(0, 0, vec).unwrap();
        let m1 = m.T();
        assert_eq!(0, m1.height);
        assert_eq!(0, m1.width);
        println!("{}", m1);

        let m = Matrix::new(1, 3, vec![1, 2, 3]).unwrap();
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(3, m1.height);
        assert_eq!(1, m1.width);

        let m = Matrix::new(3, 1, vec![1, 2, 3]).unwrap();
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(1, m1.height);
        assert_eq!(3, m1.width);
    }

    #[test]
    #[should_panic("shape size does not match")]
    fn test_reshape_panic() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = m.reshape(4, 2).unwrap();
    }
    #[test]
    fn test_reshape() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = m.reshape(3, 2).unwrap();
        assert_eq!(m1.size(), (3, 2));
        assert_eq!(m1.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    #[should_panic("shape size does not match")]
    fn test_hstack_panic() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let r = m.hstack(&m1).unwrap();
    }
    #[test]
    fn test_hstack() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = Matrix::new(2, 4, vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        let mr = m.hstack(&m1).unwrap();
        assert_eq!(mr.size(), (2, 7));
        assert_eq!(mr.data, vec![1, 2, 3, 1, 2, 3, 4, 4, 5, 6, 5, 6, 7, 8]);
    }

    #[test]
    #[should_panic("shape size does not match")]
    fn test_vstack_panic() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let r = m.vstack(&m1).unwrap();
    }

    #[test]
    fn test_vstack() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let mr = m.vstack(&m1).unwrap();
        assert_eq!(mr.size(), (5, 3));
        assert_eq!(mr.data, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
