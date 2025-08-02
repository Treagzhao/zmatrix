use crate::dense::error::OperationError;
use crate::dense::Matrix;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, ParallelBridge};
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread;
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
    pub fn T(&self) -> Matrix<COLS, ROWS, T> {
        let mut data = array_init(|i| {
            let row = i / ROWS;
            let col = i % ROWS;
            self.data[col * COLS + row]
        });
        Matrix::new(data)
    }

    pub fn reshape<const NEW_ROWS: usize, const NEW_COLS: usize>(&self) -> Result<Matrix<NEW_ROWS, NEW_COLS, T>, OperationError> {
        if ROWS * COLS != NEW_ROWS * NEW_COLS {
            return Err(OperationError {
                message: "shape size does not match".to_string(),
            });
        }
        Ok(Matrix::new(self.data.clone()))
    }

    pub fn hstack<const RHS_COLS: usize>(&self, rhs: &Matrix<ROWS, RHS_COLS, T>) -> Result<Matrix<ROWS, {COLS + RHS_COLS}, T>, OperationError> {
        let data = array_init(|i| {
            let row = i / (COLS + RHS_COLS);
            let col = i % (COLS + RHS_COLS);
            if col < COLS {
                self.data[row * COLS + col]
            } else {
                rhs.data[row * RHS_COLS + (col - COLS)]
            }
        });
        Ok(Matrix::new(data))
    }

    pub fn vstack<const RHS_ROWS: usize>(&self, rhs: &Matrix<RHS_ROWS, COLS, T>) -> Result<Matrix<{ROWS + RHS_ROWS}, COLS, T>, OperationError> {
        let data = array_init(|i| {
            if i < ROWS * COLS {
                self.data[i]
            } else {
                rhs.data[i - ROWS * COLS]
            }
        });
        Ok(Matrix::new(data))
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Send + Copy + Sync + Display + std::iter::Sum + Default + Debug + Add<Output = T>,
{
    // 计算矩阵的行和，返回一个列向量
    pub fn sum_row(&self) -> Matrix<ROWS, 1, T> {
        let mut data = [T::default(); ROWS];
        (0..ROWS).into_par_iter().for_each(|i| {
            data[i] = self.data[i * COLS..(i + 1) * COLS]
                .iter()
                .cloned()
                .sum();
        });
        Matrix::new(data)
    }

    // 计算矩阵的列和，返回一个行向量
    pub fn sum_column(&self) -> Matrix<1, COLS, T> {
        let mut data = [T::default(); COLS];
        (0..COLS).into_par_iter().for_each(|i| {
            data[i] = (0..ROWS).map(|j| self.data[j * COLS + i]).sum();
        });
        Matrix::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_T() {
        let m = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let m1 = m.T();
        assert_eq!(m1.size(), (3, 2));
        println!("{}", m1);
        assert_eq!(m1.data, [1, 4, 2, 5, 3, 6]);

        let m = Matrix::<0, 0, i32>::new([]);
        let m1 = m.T();
        assert_eq!(m1.size(), (0, 0));
        println!("{}", m1);

        let m = Matrix::<1, 3, i32>::new([1, 2, 3]);
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(m1.size(), (3, 1));

        let m = Matrix::<3, 1, i32>::new([1, 2, 3]);
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(m1.size(), (1, 3));
    }

    #[test]
    fn test_reshape() {
        let m = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let m1 = m.reshape::<3, 2>().unwrap();
        assert_eq!(m1.size(), (3, 2));
        assert_eq!(m1.data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_hstack() {
        let m = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let m1 = Matrix::<2, 4, i32>::new([1, 2, 3, 4, 5, 6, 7, 8]);
        let mr = m.hstack(&m1).unwrap();
        assert_eq!(mr.size(), (2, 7));
        assert_eq!(mr.data, [1, 2, 3, 1, 2, 3, 4, 4, 5, 6, 5, 6, 7, 8]);
    }

    #[test]
    fn test_vstack() {
        let m = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let m1 = Matrix::<3, 3, i32>::new([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mr = m.vstack(&m1).unwrap();
        assert_eq!(mr.size(), (5, 3));
        assert_eq!(mr.data, [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_sum_vertical_basic() {
        let m = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let result = m.sum_row();
        assert_eq!(result.size(), (2, 1));
        assert_eq!(result.data, [6, 15]);
    }

    #[test]
    fn test_sum_vertical_single_row() {
        let m = Matrix::<1, 4, i32>::new([1, 2, 3, 4]);
        let result = m.sum_row();
        assert_eq!(result.size(), (1, 1));
        assert_eq!(result.data, [10]);
    }

    #[test]
    fn test_sum_vertical_single_column() {
        let m = Matrix::<3, 1, i32>::new([1, 2, 3]);
        let result = m.sum_row();
        assert_eq!(result.size(), (3, 1));
        assert_eq!(result.data, [1, 2, 3]);
    }

    #[test]
    fn test_sum_vertical_large_matrix() {
        let data: [i32; 100] = (1..=100).collect::<Vec<_>>().try_into().unwrap();
        let m = Matrix::<10, 10, i32>::new(data);
        let result = m.sum_row();
        assert_eq!(result.size(), (10, 1));
        assert_eq!(result.data, [55, 155, 255, 355, 455, 555, 655, 755, 855, 955]);
    }

    #[test]
    fn test_sum_vertical_float_values() {
        let m = Matrix::<2, 2, f64>::new([1.5, 2.5, 3.5, 4.5]);
        let result = m.sum_row();
        assert_eq!(result.size(), (2, 1));
        assert_eq!(result.data, [4.0, 8.0]);
    }

    #[test]
    fn test_sum_column_basic() {
        let m = Matrix::<3, 2, i32>::new([1, 2, 3, 4, 5, 6]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 2));
        assert_eq!(result.data, [9, 12]);
    }

    #[test]
    fn test_sum_column_single_column() {
        let m = Matrix::<3, 1, i32>::new([1, 2, 3]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 1));
        assert_eq!(result.data, [6]);
    }

    #[test]
    fn test_sum_column_single_row() {
        let m = Matrix::<1, 4, i32>::new([1, 2, 3, 4]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 4));
        assert_eq!(result.data, [1, 2, 3, 4]);
    }

    #[test]
    fn test_sum_column_empty_matrix() {
        let m = Matrix::<0, 0, f64>::new([]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 0));
        assert_eq!(result.data, []);
    }

    #[test]
    fn test_sum_column_large_matrix() {
        let data: [i32; 100] = (1..=100).collect::<Vec<_>>().try_into().unwrap();
        let m = Matrix::<10, 10, i32>::new(data);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 10));
        assert_eq!(result.data, [460, 470, 480, 490, 500, 510, 520, 530, 540, 550]);
    }

    #[test]
    fn test_sum_column_float_values() {
        let m = Matrix::<2, 2, f64>::new([1.5, 2.5, 3.5, 4.5]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 2));
        assert_eq!(result.data, [5.0, 7.0]);
    }
}
