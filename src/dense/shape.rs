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

impl<T> Matrix<T>
where
    T: Send + Copy + Sync + Display + Add<Output = T>,
{
    // 计算矩阵的行和，返回一个列向量
    pub fn sum_row(&self) -> Matrix<T> {
        let mut data: Vec<T> = Vec::with_capacity(self.height);
        for i in 0..self.height {
            let mut sum: T = self.data[i * self.width];
            for j in 1..self.width {
                sum = sum + self.data[i * self.width + j];
            }
            data.push(sum);
        }
        Matrix::new(self.height, 1, data).unwrap()
    }
    // 计算矩阵的列和，返回一个行向量
    pub fn sum_column(&self) -> Matrix<T> {
        let mut data: Vec<T> = Vec::with_capacity(self.width);
        for i in 0..self.width {
            let mut sum: T = self.data[i];
            for j in 1..self.height {
                sum = sum + self.data[j * self.width + i];
            }
            data.push(sum);
        }
        Matrix::new(1, self.width, data).unwrap()
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


    #[test]
    fn test_sum_vertical_basic() {
        // 测试基本的垂直求和功能
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let result = m.sum_row();
        assert_eq!(result.height, 2);
        assert_eq!(result.width, 1);
        assert_eq!(result.data, vec![6, 15]);  // 1+2+3=6, 4+5+6=15
    }

    #[test]
    fn test_sum_vertical_single_row() {
        // 测试单行矩阵
        let m = Matrix::new(1, 4, vec![1, 2, 3, 4]).unwrap();
        let result = m.sum_row();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 1);
        assert_eq!(result.data, vec![10]);  // 1+2+3+4=10
    }

    #[test]
    fn test_sum_vertical_single_column() {
        // 测试已经是单列的矩阵
        let m = Matrix::new(3, 1, vec![1, 2, 3]).unwrap();
        let result = m.sum_row();
        assert_eq!(result.height, 3);
        assert_eq!(result.width, 1);
        assert_eq!(result.data, vec![1, 2, 3]);  // 每行只有一个元素
    }


    #[test]
    fn test_sum_vertical_large_matrix() {
        // 测试大矩阵
        let data = (1..=100).collect::<Vec<i32>>();
        let m = Matrix::new(10, 10, data).unwrap();
        let result = m.sum_row();
        assert_eq!(result.height, 10);
        assert_eq!(result.width, 1);
        assert_eq!(
            result.data,
            vec![55, 155, 255, 355, 455, 555, 655, 755, 855, 955]
        );
    }

    #[test]
    fn test_sum_vertical_float_values() {
        // 测试浮点数矩阵
        let m = Matrix::new(2, 2, vec![1.5, 2.5, 3.5, 4.5]).unwrap();
        let result = m.sum_row();
        assert_eq!(result.height, 2);
        assert_eq!(result.width, 1);
        assert_eq!(result.data, vec![4.0, 8.0]);  // 1.5+2.5=4.0, 3.5+4.5=8.0
    }


    #[test]
    fn test_sum_column_basic() {
        // 测试基本的列求和功能
        let m = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 2);
        assert_eq!(result.data, vec![9, 12]);  // 1+3+5=9, 2+4+6=12
    }

    #[test]
    fn test_sum_column_single_column() {
        // 测试单列矩阵
        let m = Matrix::new(3, 1, vec![1, 2, 3]).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 1);
        assert_eq!(result.data, vec![6]);  // 1+2+3=6
    }

    #[test]
    fn test_sum_column_single_row() {
        // 测试单行矩阵
        let m = Matrix::new(1, 4, vec![1, 2, 3, 4]).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 4);
        assert_eq!(result.data, vec![1, 2, 3, 4]);  // 每列只有一个元素
    }

    #[test]
    fn test_sum_column_empty_matrix() {
        // 测试空矩阵
        let m:Matrix<f64> = Matrix::new(0, 0, vec![], ).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 0);
        assert_eq!(result.data, vec![]);
    }

    #[test]
    fn test_sum_column_large_matrix() {
        // 测试大矩阵
        let data = (1..=100).collect::<Vec<i32>>();
        let m = Matrix::new(10, 10, data).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 10);
        assert_eq!(
            result.data,
            vec![460, 470, 480, 490, 500, 510, 520, 530, 540, 550]
        );
    }

    #[test]
    fn test_sum_column_float_values() {
        // 测试浮点数矩阵
        let m = Matrix::new(2, 2, vec![1.5, 2.5, 3.5, 4.5]).unwrap();
        let result = m.sum_column();
        assert_eq!(result.height, 1);
        assert_eq!(result.width, 2);
        assert_eq!(result.data, vec![5.0, 7.0]);  // 1.5+3.5=5.0, 2.5+4.5=7.0
    }
}
