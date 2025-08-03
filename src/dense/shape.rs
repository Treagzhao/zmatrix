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
        let mut data = [[T::default(); ROWS]; COLS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[col][row] = self.data[row][col];
            }
        }
        Matrix::new(data)
    }

    pub fn reshape<const NEW_ROWS: usize, const NEW_COLS: usize>(&self) -> Result<Matrix<NEW_ROWS, NEW_COLS, T>, OperationError> {
        if ROWS * COLS != NEW_ROWS * NEW_COLS {
            return Err(OperationError {
                message: "shape size does not match".to_string(),
            });
        }
        let mut new_data = [[T::default(); NEW_COLS]; NEW_ROWS];
        for i in 0..(ROWS * COLS) {
            let old_row = i / COLS;
            let old_col = i % COLS;
            let new_row = i / NEW_COLS;
            let new_col = i % NEW_COLS;
            new_data[new_row][new_col] = self.data[old_row][old_col];
        }
        Ok(Matrix::new(new_data))
    }
    //
    // pub fn hstack<const RHS_COLS: usize>(&self, rhs: &Matrix<ROWS, RHS_COLS, T>) -> Result<Matrix<ROWS, {COLS + RHS_COLS}, T>, OperationError> {
    //     let mut data = [[T::default(); COLS + RHS_COLS]; ROWS];
    //     for row in 0..ROWS {
    //         for col in 0..COLS {
    //             data[row][col] = self.data[row][col];
    //         }
    //         for col in 0..RHS_COLS {
    //             data[row][COLS + col] = rhs.data[row][col];
    //         }
    //     }
    //     Ok(Matrix::new(data))
    // }
    //
    // pub fn vstack<const RHS_ROWS: usize>(&self, rhs: &Matrix<RHS_ROWS, COLS, T>) -> Result<Matrix<{ROWS + RHS_ROWS}, COLS, T>, OperationError> {
    //     let mut data: [[T; COLS]; ROWS + RHS_ROWS] = vec![vec![T::default(); COLS].try_into().unwrap(); ROWS + RHS_ROWS].try_into().unwrap();
    //     for row in 0..ROWS {
    //         for col in 0..COLS {
    //             data[row][col] = self.data[row][col];
    //         }
    //     }
    //     for row in 0..RHS_ROWS {
    //         for col in 0..COLS {
    //             data[ROWS + row][col] = rhs.data[row][col];
    //         }
    //     }
    //     Ok(Matrix::new(data))
    // }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Send + Copy + Sync + Display + std::iter::Sum + Default + Debug + Add<Output = T>,
{
    // 计算矩阵的行和，返回一个列向量
    pub fn sum_row(&self) -> Matrix<ROWS, 1, T> {
        let mut data = [[T::default(); 1]; ROWS];
        for row in 0..ROWS {
            data[row][0] = self.data[row].iter().cloned().sum();
        }
        Matrix::new(data)
    }

    // 计算矩阵的列和，返回一个行向量
    pub fn sum_column(&self) -> Matrix<1, COLS, T> {
        let mut data = [[T::default(); COLS]; 1];
        for col in 0..COLS {
            data[0][col] = (0..ROWS).map(|row| self.data[row][col]).sum();
        }
        Matrix::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_T() {
        let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m1 = m.T();
        assert_eq!(m1.size(), (3, 2));
        println!("{}", m1);
        assert_eq!(m1.data, [[1, 4], [2, 5], [3, 6]]);

        let m = Matrix::<0, 0, i32>::new([[]; 0]);
        let m1 = m.T();
        assert_eq!(m1.size(), (0, 0));
        println!("{}", m1);

        let m = Matrix::<1, 3, i32>::new([[1, 2, 3]]);
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(m1.size(), (3, 1));

        let m = Matrix::<3, 1, i32>::new([[1], [2], [3]]);
        let m1 = m.T();
        println!("{}", m1);
        assert_eq!(m1.size(), (1, 3));
    }

    #[test]
    fn test_reshape() {
        let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m1 = m.reshape::<3, 2>().unwrap();
        assert_eq!(m1.size(), (3, 2));
        assert_eq!(m1.data, [[1, 2], [3, 4], [5, 6]]);
    }

    // #[test]
    // fn test_hstack() {
    //     let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
    //     let m1 = Matrix::<2, 4, i32>::new([[1, 2, 3, 4], [5, 6, 7, 8]]);
    //     let mr = m.hstack(&m1).unwrap();
    //     assert_eq!(mr.size(), (2, 7));
    //     assert_eq!(mr.data, [[1, 2, 3, 1, 2, 3, 4], [4, 5, 6, 5, 6, 7, 8]]);
    // }
    //
    // #[test]
    // fn test_vstack() {
    //     let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
    //     let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    //     let mr = m.vstack(&m1).unwrap();
    //     assert_eq!(mr.size(), (5, 3));
    //     assert_eq!(mr.data, [[1, 2, 3], [4, 5, 6], [1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    // }

    #[test]
    fn test_sum_vertical_basic() {
        let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let result = m.sum_row();
        assert_eq!(result.size(), (2, 1));
        assert_eq!(result.data, [[6], [15]]);
    }

    #[test]
    fn test_sum_vertical_single_row() {
        let m = Matrix::<1, 4, i32>::new([[1, 2, 3, 4]]);
        let result = m.sum_row();
        assert_eq!(result.size(), (1, 1));
        assert_eq!(result.data, [[10]]);
    }

    #[test]
    fn test_sum_vertical_single_column() {
        let m = Matrix::<3, 1, i32>::new([[1], [2], [3]]);
        let result = m.sum_row();
        assert_eq!(result.size(), (3, 1));
        assert_eq!(result.data, [[1], [2], [3]]);
    }

    #[test]
    fn test_sum_vertical_large_matrix() {
        let data = (1..=100).collect::<Vec<_>>();
        let m = Matrix::<10, 10, i32>::new([
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
            [31, 32, 33, 34, 35, 36, 37, 38, 39, 40],
            [41, 42, 43, 44, 45, 46, 47, 48, 49, 50],
            [51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            [61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
            [71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
            [91, 92, 93, 94, 95, 96, 97, 98, 99, 100],
        ]);
        let result = m.sum_row();
        assert_eq!(result.size(), (10, 1));
        assert_eq!(result.data, [[55], [155], [255], [355], [455], [555], [655], [755], [855], [955]]);
    }

    #[test]
    fn test_sum_vertical_float_values() {
        let m = Matrix::<2, 2, f64>::new([[1.5, 2.5], [3.5, 4.5]]);
        let result = m.sum_row();
        assert_eq!(result.size(), (2, 1));
        assert_eq!(result.data, [[4.0], [8.0]]);
    }

    #[test]
    fn test_sum_column_basic() {
        let m = Matrix::<3, 2, i32>::new([[1, 2], [3, 4], [5, 6]]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 2));
        assert_eq!(result.data, [[9, 12]]);
    }

    #[test]
    fn test_sum_column_single_column() {
        let m = Matrix::<3, 1, i32>::new([[1], [2], [3]]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 1));
        assert_eq!(result.data, [[6]]);
    }

    #[test]
    fn test_sum_column_single_row() {
        let m = Matrix::<1, 4, i32>::new([[1, 2, 3, 4]]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 4));
        assert_eq!(result.data, [[1, 2, 3, 4]]);
    }

    #[test]
    fn test_sum_column_empty_matrix() {
        let m = Matrix::<0, 0, f64>::new([[]; 0]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 0));
        assert_eq!(result.data, [[]]);
    }

    #[test]
    fn test_sum_column_large_matrix() {
        let m = Matrix::<10, 10, i32>::new([
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
            [31, 32, 33, 34, 35, 36, 37, 38, 39, 40],
            [41, 42, 43, 44, 45, 46, 47, 48, 49, 50],
            [51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            [61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
            [71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
            [91, 92, 93, 94, 95, 96, 97, 98, 99, 100],
        ]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 10));
        assert_eq!(result.data, [[460, 470, 480, 490, 500, 510, 520, 530, 540, 550]]);
    }

    #[test]
    fn test_sum_column_float_values() {
        let m = Matrix::<2, 2, f64>::new([[1.5, 2.5], [3.5, 4.5]]);
        let result = m.sum_column();
        assert_eq!(result.size(), (1, 2));
        assert_eq!(result.data, [[5.0, 7.0]]);
    }
}
