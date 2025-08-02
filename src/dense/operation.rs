use crate::dense::{error, util, Matrix};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{mpsc, Arc};
use std::thread;
use approx::assert_relative_eq;
use array_init::array_init;

pub struct Scalar<T> {
    value: T,
}

impl<T> Scalar<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Add for Matrix<ROWS, COLS, T>
where
    T: Copy + Add<Output = T> + Display + Send + Sync + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn add(self, other: Self) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] + other.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Sub for Matrix<ROWS, COLS, T>
where
    T: Copy + Sub<Output = T> + Display + Default+ Send + Sync,
{
    type Output = Matrix<ROWS, COLS, T>;
    fn sub(self, other: Self) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] - other.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Mul for Matrix<ROWS, COLS, T>
where
    T: Copy + Mul<Output = T> + Default+Display + Send + Sync,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn mul(self, other: self) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] * other.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Neg for Matrix<ROWS, COLS, T>
where
    T: Copy + Neg<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn neg(self) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = -self.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync + Into<f64> + Display,
{
    pub fn exp(&self) -> Matrix<ROWS, COLS, f64> {
        let mut data = [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col].into().exp();
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Add<T> for Matrix<ROWS, COLS, T>
where
    T: Copy + Add<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] + rhs;
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Sub<T> for Matrix<ROWS, COLS, T>
where
    T: Copy + Sub<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;
    fn sub(self, rhs: T) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] - rhs;
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Mul<T> for Matrix<ROWS, COLS, T>
where
    T: Copy + Mul<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] * rhs;
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Div<T> for Matrix<ROWS, COLS, T>
where
    T: Copy + Div<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;
    fn div(self, rhs: T) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] / rhs;
            }
        }
        Matrix { data, digits: 0 }
    }
}
impl<const ROWS: usize, const COLS: usize, T> Sub<Matrix<ROWS, COLS, T>> for Scalar<T>
where
    T: Copy + Sub<Output = T> + Send + Sync + Display + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn sub(self, rhs: Matrix<ROWS, COLS, T>) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.value - rhs.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Default + Display + Send + Sync + Copy + Add<Output = T> + Mul<Output = T> + Sum,
{
    pub fn product<const COLS2:usize>(&self, target: &Matrix<COLS, COLS2, T>) -> Result<Matrix<ROWS, COLS2, T>, error::OperationError> {
        let mut data = [[T::default(); COLS2]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLS2 {
                let mut sum = T::default();
                for k in 0..COLS {
                    sum = sum + self.data[i][k] * target.data[k][j];
                }
                data[i][j] = sum;
            }
        }
        Ok(Matrix { data, digits: 0 })
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Display + Send + Sync + Mul<Output = T> + Copy + Default,
{
    pub fn scale(&self, scalar: T) -> Matrix<ROWS, COLS, T> {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col] * scalar;
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Div<Matrix<ROWS, COLS, T>> for Scalar<T>
where
    T: Display + Send + Sync + Div<Output = T> + Copy + Default,
{
    type Output = Matrix<ROWS, COLS, T>;

    fn div(self, rhs: Matrix<ROWS, COLS, T>) -> Self::Output {
        let mut data = [[T::default(); COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.value / rhs.data[row][col];
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync + Display + Into<f64>,
{
    pub fn ln(&self) -> Matrix<ROWS, COLS, f64> {
        let mut data = [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col].into().ln();
            }
        }
        Matrix { data, digits: 0 }
    }

    pub fn clamp(&self, min: T, max: T) -> Matrix<ROWS, COLS, f64> {
        let min_f: f64 = min.into();
        let max_f: f64 = max.into();
        let mut data = [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                data[row][col] = self.data[row][col].into().clamp(min_f, max_f);
            }
        }
        Matrix { data, digits: 0 }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync + Display + Add<Output = T> + Default,
{
    pub fn sum(&self) -> T {
        if ROWS == 0 || COLS == 0 {
            return T::default();
        }
        let mut sum = T::default();
        for row in 0..ROWS {
            for col in 0..COLS {
                sum = sum + self.data[row][col];
            }
        }
        sum
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync + Display + Div<Output = T> + Add<Output = T> + Default,
    f64: From<T>,
{
    pub fn mean(&self) -> f64 {
        if ROWS == 0 || COLS == 0 {
            return 0.0;
        }
        let sum = self.sum();
        let s: f64 = sum.into();
        s / (ROWS * COLS) as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use std::panic;
    #[test]
    fn test_add_ok() {
        let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::<2, 3, i32>::new([[12, 22, 33], [44, 51, 56]]);
        let m3 = m1 + m2;
        assert_eq!([[13, 24, 36], [48, 56, 62]], m3.data);
    }


    #[test]
    fn test_sub() {
        let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::<2, 3, i32>::new([[12, 22, 33], [44, 51, 56]]);
        let m3 = m2 - m1;
        assert_eq!([[11, 20, 30], [40, 46, 50]], m3.data);
    }


    #[test]
    fn test_mul() {
        let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::<2, 3, i32>::new([[2, 3, 4], [5, 6, 7]]);
        let m3 = m1 * m2;
        assert_eq!([[2, 6, 12], [20, 30, 42]], m3.data);
    }

    #[test]
    fn test_matrix_product() {
        let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::<3, 4, i32>::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
        let result = m1.product(&m2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().data, [[38, 44, 50, 56], [83, 98, 113, 128]]);
    }


    #[test]
    fn test_neg() {
        let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = -m1;
        assert_eq!(2, m2.height());
        assert_eq!(3, m2.width());
        assert_eq!([[-1, -2, -3], [-4, -5, -6]], m2.data);
    }

    #[test]
    fn test_exp_positive_values() {
        let matrix = Matrix::<2, 2, f64>::new([[1.0, 2.0], [0.5, 3.0]]);
        let result = matrix.exp();

        assert_eq!(result.height(), 2);
        assert_eq!(result.width(), 2);
        assert_relative_eq!(result.data[0], 1.0f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 2.0f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 0.5f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[3], 3.0f64.exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_div_zero() {
        // Test division by zero
        let m = Matrix::<1, 2, f64>::new([[1.0, 2.0]]);
        let result = m / 0.0;
        assert!(result.data[0][0].is_infinite());
        assert!(result.data[0][1].is_infinite());
    }

    #[test]
    fn test_scalar_sub_matrix() {
        // Test scalar minus matrix
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let result = Scalar::new(10) - m;
        assert_eq!(result.data, [[9, 8], [7, 6]]);

        // Test with floating point
        let m = Matrix::<1, 3, f64>::new([[1.5, 2.5, 3.5]]);
        let result = Scalar::new(10.0) - m;
        assert_relative_eq!(result.data[0][0], 8.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[0][1], 7.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[0][2], 6.5, epsilon = 1e-10);
    }
    }

    #[test]
    fn test_exp_large_values() {
        let matrix = Matrix::<1, 2, f64>::new([[10.0, 20.0]]);
        let result = matrix.exp();

        assert_eq!(result.height(), 1);
        assert_eq!(result.width(), 2);
        assert_relative_eq!(result.data[0], 10.0f64.exp(), epsilon = 1e10); // Large epsilon for big numbers
        assert_relative_eq!(result.data[1], 20.0f64.exp(), epsilon = 1e10);
    }

    #[test]
    fn test_exp_small_values() {
        let matrix = Matrix::<1, 2, f64>::new([[1e-10, -1e-10]]);
        let result = matrix.exp();

        assert_eq!(result.height(), 1);
        assert_eq!(result.width(), 2);
        assert_relative_eq!(result.data[0], (1e-10f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], (-1e-10f64).exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_single_element_matrix() {
        let matrix = Matrix::<1, 1, f64>::new([[2.0]]);
        let result = matrix.exp();

        assert_eq!(result.height(), 1);
        assert_eq!(result.width(), 1);
        assert_relative_eq!(result.data[0], 2.0f64.exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_preserves_dimensions() {
        let matrix = Matrix::<3, 4, f64>::new([[1.0; 4]; 3]);
        let result = matrix.exp();

        assert_eq!(result.height(), 3);
        assert_eq!(result.width(), 4);
        assert_eq!(result.data.len(), 12);
    }

    #[test]
    fn test_exp_with_nan() {
        let matrix = Matrix::<1, 1, f64>::new([f64::NAN]);
        let result = matrix.exp();

        assert!(result.data[0].is_nan());
    }

    #[test]
    fn test_exp_with_infinity() {
        let matrix = Matrix::<1, 2, f64>::new([[f64::INF极INITY, f64::NEG_INFINITY]]);
        let result = matrix.exp();

        assert_eq!(result.data[0], f64::INFINITY);
        assert_eq!(result.data[1], 0.0);
    }

    #[test]
    fn test_add_scalar() {
        // Test basic addition with scalar
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let result = m + 5;
        assert_eq!(result.data, [[6, 7], [8, 9]]);
        assert_eq!(result.height(), 2);
        assert_eq!(result.width(), 2);

        // Test with negative scalar
        let m = Matrix::<2, 2, i32>::new([[10, 20], [30, 40]]);
        let result = m + (-5);
        assert_eq!(result.data, [[5, 15], [25, 35]]);
    }

    #[test]
    fn test_add_scalar_float() {
        // Test with floating point numbers
        let m = Matrix::<1, 3, f64>::new([[1.5, 2.5, 3.5]]);
        let result = m + 1.5;
        assert_relative_eq!(result.data[0][0], 3.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[0][1], 4.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[0][2], 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sub_scalar() {
        // Test basic subtraction with scalar
        let m = Matrix::<2, 2, i32>::new([[10, 20], [30, 40]]);
        let result = m - 5;
        assert_eq!(result.data, [[5, 15], [25, 35]]);
        let m = Matrix::<2, 2, i32>::new([[10, 20], [30, 40]]);
        // Test with negative scalar
        let result = m - (-5);
        assert_eq!(result.data, [[15, 25], [35, 45]]);
    }

    #[test]
    fn test_mul_scalar() {
        // Test basic multiplication with scalar
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let result = m * 5;
        assert_eq!(result.data, [[5, 10], [15, 20]]);

        // Test with zero
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let result = m * 0;
        assert_eq!(result.data, [[0, 0], [0, 0]]);

        // Test with negative scalar
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let result = m * -1;
        assert_eq!(result.data, [[-1, -2], [-3, -4]]);
    }

    #[test]
    fn test_div_scalar() {
        // Test basic division with scalar
        let m = Matrix::<2, 2, f64>::new([[10.0, 20.0], [30.0, 40.0]]);
        let result = m / 2.0;
        assert_relative_eq!(result.data[0][0], 5.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[0][1], 10.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1][0], 15.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1][1], 20.极， epsilon = 1e-10);
    }

    #[test]
    fn test_div_zero() {
        // Test division by zero
        let m = Matrix::<1, 2, f64>::new([1.0, 2.0]);
        let result = m / 0.0;
        assert!(result.data[0].is_infinite());
        assert!(result.data[1].is_infinite());
    }

    #[test]
    fn test_scalar_sub_matrix() {
        // Test scalar minus matrix
        let m = Matrix::<2, 2, i32>::new([1, 2, 3, 4]);
        let result = Scalar::new(10) - m;
        assert_eq!(result.data, [9, 8, 7, 6]);

        // Test with floating point
        let m = Matrix::<1, 3, f64>::new([1.5, 2.5, 3.5]);
        let result = Scalar::new(10.0) - m;
        assert_relative_eq!(result.data[0], 8.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 7.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 6.5, epsilon = 1e-10);
    }

    #[极，t]
    fn test_edge_cases() {
        // Test empty matrix
        let m = Matrix::<0, 0, i32>::new([[]]);
        let result = m + 10;
        assert_eq!(result.data, []);

        // Test single element matrix
        let m = Matrix::<1, 1, i32>::new([[5]]);
        let result = m * 2;
        assert_eq!(result.data, [[10]]);

        // Test large values
        let m = Matrix::<1, 2, i32>::new([[i32::MAX, i32::MIN]]);
        let result = panic::catch_unwind(|| m + 1);
        assert!(result.is_err()); // Overflow check
    }

    #[test]
    fn test_scale() {
        let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m1 = m.scale(2);
        assert_eq!(2, m1.height());
        assert_eq!(3, m1.width());
        assert_eq!([[2, 4, 6], [8, 10, 12]], m1.data);
    }

    #[test]
    fn test_new_with_integer() {
        // Test creating Scalar with integer value
        let scalar = Scalar::new(42);
        assert_eq!(scalar.value, 42);
    }

    #[test]
    fn test_new_with_float() {
        // Test creating Scalar with float value
        let scalar = Scalar::new(3.14);
        assert_eq!(scalar.value, 3.14);
    }

    #[test]
    fn test_div_by_num() {
        let m = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let m1 = Scalar::new(2) / m;
        assert_eq!(2, m1.height());
        assert_eq!(3, m1.width());
        assert_eq!([[2, 1, 0], [0, 0, 0]], m1.data);

        let m = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let m1 = Scalar::new(2.0) / m;
        assert_eq!(2, m极.height());
        assert_eq!(3, m1.width());
        assert_eq!(
            [[2.0, 1.0, 0.6666666666666666], [0.5, 0.4, 0.3333333333333333]],
            m1.data
        );
    }

    #[test]
    fn test_log_positive_values() {
        let matrix = Matrix::<2, 2, f64>::new([1.0, 2.0, 3.0, 4.0]);
        let result = matrix.ln();

        assert_eq!(result.height(), 2);
        assert_eq!(result.width(), 2);
        assert_relative_eq!(result.data[0], 1.0f64.ln(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 2.0f64.ln(), epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 3.0f64.ln(), epsilon = 1e-10);
        assert_relative_eq!(result.data[3], 4.0f64.ln(), epsilon = 1e-10);
    }

    #[test]
    fn test_log_fractional_values() {
        let matrix = Matrix::<1, 3, f64>::new([0.5, 0.25, 0.125]);
        let result = matrix.ln();

        assert_eq!(result.height(), 1);
        assert_eq!(result.width(), 3);
        assert_relative_eq!(result.data[0], 0.5f64.ln(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 0.25f64.ln(), epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 0.125f64.ln(), epsilon = 1e-10);
    }

    #[test]
    fn test_log_zero_values() {
        let matrix = Matrix::<2, 2, f64>::new([0.0, 0.0, 0.0, 0.0]);
        let r = matrix.ln();
        assert!(r.data[0].is_infinite());
        assert!(r.data[1].is_infinite());
        assert!(r.data[2].is_infinite());
        assert!(r.data[3].is_infinite());
    }

    #[test]
    fn test_log_negative_values() {
        let matrix = Matrix::<1, 2, f64>::new([-1.0, -2.0]);
        let r = matrix.ln();
        assert!(r.data[0].is_nan());
        assert!(r.data[1].is_nan());
    }

    #[test]
    fn test_log_preserves_dimensions() {
        let matrix = Matrix::<3, 4, f64>::new([1.1; 12]);
        let result = matrix.ln();

        assert_eq!(result.height(), 3);
        assert_eq!(result.width(), 4);
        assert_eq!(result.data.len(), 12);
    }

    #[test]
    fn test_log_single_element_matrix() {
        let matrix = Matrix::<1, 1, f64>::new([2.718281828459045]);
        let result = matrix.ln();

        assert_eq!(result.height(), 1);
        assert_eq!(result.width(), 1);
        assert_relative_eq!(result.data[0], 1.0, epsilon = 1e-10); // ln(e) = 1
    }

    #[test]
    fn test_log_with_nan() {
        let matrix = Matrix::<1, 1, f64>::new([f64::NAN]);
        let r = matrix.ln();
        assert!(r.data[0].is_nan());
    }

    #[test]
    fn test_log_with_infinity() {
        let matrix = Matrix::<1, 2, f64>::new([f64::INFINITY, f64::NEG_INFINITY]);
        let r = matrix.ln();
        assert!(r.data[0].is_infinite());
        assert!(r.data[1].is_nan());
    }

    #[test]
    fn test_log_large_values() {
        let matrix = Matrix::<1, 2, f64>::new([1e50, 1e100]);
        let result = matrix.ln();

        assert_relative_eq!(
            result.data[0],
            50.0 * std::f64::consts::LN_10,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            result.data[1],
            100.0 * std::f64::consts::LN_10,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_log_small_positive_values() {
        let matrix = Matrix::<1, 2, f64>::new([1e-50, 1e-100]);
        let result = matrix.ln();

        assert_relative_eq!(
            result.data[0],
            -50.0 * std::f64::consts::LN_10,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            result.data[1],
            -100.0 * std::f64::consts::LN_10,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_log_edge_case_near_one() {
        let matrix = Matrix::<1, 2, f64>::new([1.0 - 1e-10, 1.0 + 1e-10]);
        let result = matrix.ln();

        // Test values very close to 1.0
        assert_relative_eq!(result.data[0], (1.0 - 1e-10f64).ln(), epsilon = 1e-6); // More tolerant epsilon
        assert_relative_eq!(result.data[1], (1.0 + 1e-10f64).ln(), epsilon = 1e-6);
    }

    #[test]
    fn test_sum_empty_matrix() {
        // Test with empty matrix (should panic)
        let matrix = Matrix::<0, 0, i32>::new([]);
        let sum = matrix.sum();
        assert_eq!(0, sum);
    }

    #[test]
    fn test_sum_single_element() {
        // Test with single element matrix
        let matrix = Matrix::<1, 1, i32>::new([5]);
        let sum = matrix.sum();
        assert_eq!(sum, 5);

        // Test with negative value
        let matrix = Matrix::<1, 1, i32>::new([-3]);
        let sum = matrix.sum();
        assert_eq!(sum, -3);
    }

    #[test]
    fn test_sum_positive_integers() {
        // Test with positive integers
        let matrix = Matrix::<2, 3, i32>::new([1, 2, 3, 4, 5, 6]);
        let sum = matrix.sum();
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_sum_negative_integers() {
        // Test with mixed positive/negative integers
        let matrix = Matrix::<2, 2, i32>::new([1, -2, 3, -4]);
        let sum = matrix.sum();
        assert_eq!(sum, -2);

        // Test with all negative
        let matrix = Matrix::<1, 3, i32>::new([-10, -20, -30]);
        let sum = matrix.sum();
        assert_eq!(sum, -60);
    }

    #[test]
    fn test_sum_floating_point() {
        // Test with floating point values
        let matrix = Matrix::<1, 4, f64>::new([1.5, 2.5, 3.5, 4.5]);
        let sum = matrix.sum();
        assert_relative_eq!(sum, 12.0, epsilon = 1e-10);

        // Test with mixed positive/negative floats
        let matrix = Matrix::<2, 2, f64>::new([-1.5, 2.5, -3.5, 4.5]);
        let sum = matrix.sum();
        assert_relative_eq!(sum, 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sum_near_overflow() {
        // Test values near overflow boundaries
        let matrix = Matrix::<1, 2, i32>::new([i32::MAX - 5, 5]);
        let sum = matrix.sum();
        assert_eq!(sum, i32::MAX);

        let matrix = Matrix::<1, 2, i32>::new([i32::MIN + 5, -5]);
        let sum = matrix.sum();
        assert_eq!(sum, i32::MIN);
    }

    #[test]
    fn test_sum_multiple_calls_consistency() {
        // Multiple calls should return same result
        let matrix = Matrix::<2, 2, i32>::new([10, 20, 30, 40]);
        let sum1 = matrix.sum();
        let sum2 = matrix.sum();
        assert_eq!(sum1, sum2);
        assert_eq!(sum1, 100);
    }

    #[test]
    fn test_sum_with_zeroes() {
        // All zeros shouldn't affect sum
        let matrix = Matrix::<3, 3, i32>::new([0; 9]);
        let sum = matrix.sum();
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_sum_with_large_matrix() {
        // Test with many elements (performance check)
        let matrix = Matrix::<20, 50, i32>::new([1; 1000]);
        let sum = matrix.sum();
        assert_eq!(sum, 1000);
    }

    #[test]
    fn test_sum_with_nan() {
        // Test with NaN values
        let matrix = Matrix::<1, 3, f64>::new([1.0, f64::NAN, 2.0]);
        let sum = matrix.sum();
        assert!(sum.is_nan());
    }

    #[test]
    fn test_sum_with_infinity() {
        // Test with infinity values
        let matrix = Matrix::<1, 3, f64>::new([1.0, f64::INFINITY, 2.0]);
        let sum = matrix.sum();
        assert_eq!(sum, f64::INFINITY);

        // Mixed +inf and -inf
        let matrix = Matrix::<1, 2, f64>::new([f64::INFINITY, f64::NEG_INFINITY]);
        let sum = matrix.sum();
        assert!(sum.is_nan());
    }

    #[test]
    fn test_sum_with_extreme_floats() {
        // Test with extremely large/small float values
        let matrix = Matrix::<1, 2, f64>::new([f64::MIN, f64::MAX]);
        let sum = matrix.sum();
        assert_relative_eq!(sum, f64::MIN + f64::MAX, epsilon = 1e-10);
    }

    #[test]
    fn test_clamp_basic_values() {
        // Test basic clamping with normal values
        let matrix = Matrix::<2, 2, f64>::new([1.0, 2.5, 3.0, 4.5]);
        let result = matrix.clamp(2.0, 4.0);
        assert_eq!(result.height(), 2);
        assert_eq!(result.width(), 2);
        assert_relative_eq!(result.data[0], 2.0, epsilon = 1e-10); // 1.0 clamped to 2.0
        assert_relative_eq!(result.data[1], 2.5, epsilon = 1e-10); // unchanged
        assert_relative_eq!(result.data[2], 3.0, epsilon = 1e-10); // unchanged
        assert_relative_eq!(result.data[3], 4.0, epsilon = 1e-10); // 4.5 clamped to 4.0
    }

    #[test]
    fn test_clamp_all_values_below_min() {
        // Test all values below minimum
        let matrix = Matrix::<1, 3, f64>::new([0.5, 1.0, 1.5]);
        let result = matrix.clamp(2.0, 4.0);
        assert_eq!(result.data.len(), 3);
        assert_relative_eq!(result.data[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 2.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_clamp_all_values_above_max() {
        // Test all values above maximum
        let matrix = Matrix::<1, 3, f64>::new([5.0, 6.0, 7.0]);
        let result = matrix.clamp(2.0, 4.0);
        assert_eq!(result.data.len(), 3);
        assert_relative_eq!(result.data[0], 4.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 4.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_clamp_with_equal_bounds() {
        // Test with min == max
        let matrix = Matrix::<2, 2, f64>::new([1.0, 2.0, 3.0, 4.0]);
        let result = matrix.clamp(3.0, 3.0);
        assert_eq!(result.data.len(), 4);
        assert_relative_eq!(result.data[0], 3.0, epsilon = 1e-10); // clamped
        assert_relative_eq!(result.data[1], 3.0, epsilon = 1e-10); // clamped
        assert_relative_eq!(result.data[2], 3.0, epsilon = 1e-10); // unchanged
        assert_relative_eq!(result.data[3], 3.0, epsilon = 1e-10); // clamped
    }

    #[test]
    #[should_panic]
    fn test_clamp_with_reversed_bounds() {
        // Test with min > max (should clamp to max)
        let matrix = Matrix::<1, 2, f64>::new([1.5, 2.5]);
        let result = matrix.clamp(3.0, 2.0);
    }

    #[test]
    fn test_clamp_with_extreme_values() {
        // Test with extreme values (infinity, very large/small numbers)
        let matrix = Matrix::<1, 4, f64>::new([f64::MIN, f64::MAX, f64::NEG_INFINITY, f64::INFINITY]);
        let result = matrix.clamp(-1e100, 1e100);
        assert_eq!(result.data.len(), 4);
        assert_relative_eq!(result.data[0], -1e100, epsilon = 1e10); // MIN clamped to -1e100
        assert_relative_eq!(result.data[1], 1e100, epsilon = 1e10); // MAX clamped to 1e100
        assert_relative_eq!(result.data[2], -1e100, epsilon = 1e10); // -INF clamped
        assert_relative_eq!(result.data[3], 1e100, epsilon = 1e10); // INF clamped
    }

    #[test]
    fn test_clamp_with_nan_values() {
        // Test with NaN values (should propagate NaN)
        let matrix = Matrix::<1, 2, f64>::new([1.0, f64::NAN]);
        let result = matrix.clamp(0.0, 2.0);
        assert_eq!(result.data.len(), 2);
        assert_relative_eq!(result.data[0], 1.0, epsilon = 1e-10);
        assert!(result.data[1].is_nan());
    }

    #[test]
    fn test_clamp_empty_matrix() {
        // Test with empty matrix
        let matrix = Matrix::<0, 0, f64>::new([]);
        let result = matrix.clamp(1.0, 2.0);
        assert_eq!(result.height(), 0);
        assert_eq!(result.width(), 0);
        assert_eq!(result.data.len(), 0);
    }

    #[test]
    fn test_clamp_preserves_dimensions() {
        // Test that dimensions are preserved
        let matrix = Matrix::<3, 5, f64>::new([0.0; 15]);
        let result = matrix.clamp(-1.0, 1.0);
        assert_eq!(result.height(), 3);
        assert_eq!(result.width(), 5);
        assert_eq!(result.data.len(), 15);
    }

    #[test]
    fn test_clamp_with_integer_matrix() {
        // Test with integer matrix (should convert to f64)
        let matrix = Matrix::<1, 4, i32>::new([0, 1, 2, 3]);
        let result = matrix.clamp(1, 2);
        assert_eq!(result.data.len(), 4);
        assert_relative_eq!(result.data[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 2.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[3], 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_basic_values() {
        // Test basic mean calculation
        let matrix = Matrix::<2, 2, f64>::new([1.0, 2.0, 3.0, 4.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 2.5, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_negative_values() {
        // Test with negative values
        let matrix = Matrix::<1, 3, f64>::new([-1.0, 0.0, 1.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_empty_matrix() {
        // Test with empty matrix (should panic)
        let matrix = Matrix::<0, 0, f64>::new([]);
        let v = matrix.mean();
        assert_eq!(0.0, v);
    }

    #[test]
    fn test_mean_single_element() {
        // Test with single element matrix
        let matrix = Matrix::<1, 1, f64>::new([42.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 42.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_large_values() {
        // Test with large values
        let matrix = Matrix::<1, 2, f64>::new([1e100, 2e100]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 1.5e100, epsilon = 1e10);
    }

    #[test]
    fn test_mean_small_values() {
        // Test with very small values
        let matrix = Matrix::<1, 2, f64>::new([1e-100, 2e-100]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 1.5e-100, epsilon = 1e-110);
    }

    #[test]
    fn test_mean_mixed_values() {
        // Test with mixed positive/negative values
        let matrix = Matrix::<1, 4, f64>::new([1.0, -2.0, 3.0, -4.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, -0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_integer_values() {
        // Test with integer values
        let matrix = Matrix::<2, 2, i32>::new([1, 3, 5, 7]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_large_matrix() {
        // Test with large matrix
        let matrix = Matrix::<20, 50, f64>::new([1.0; 1000]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_with_nan() {
        // Test with NaN values
        let matrix = Matrix::<1, 2, f64>::new([1.0, f64::NAN]);
        let r = matrix.mean(); // Will panic when converting NAN to f64
        assert_eq!(true, r.is_nan())
    }

    #[test]
    fn test_mean_with_infinity() {
        // Test with infinity values
        let matrix = Matrix::<1, 2, f64>::new([1.0, f64::INFINITY]);
        let r = matrix.mean(); // Will panic when converting INFINITY to f64
        assert_eq!(true, r.is_infinite())
    }

    #[test]
    fn test_mean_precision_check() {
        // Test precision of mean calculation
        let matrix = Matrix::<1, 100, f64>::new([1.23; 100]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 1.23, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_odd_elements() {
        // Test with odd number of elements
        let matrix = Matrix::<1, 5, f64>::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_even_elements() {
        // Test with even number of elements
        let matrix = Matrix::<1, 4, f64>::new([1.0, 2.0, 3.0, 4.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 2.5, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_non_square_matrix() {
        // Test with non-square matrix
        let matrix = Matrix::<3, 2, f64>::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let mean = matrix.mean();
        assert_relative_eq!(mean, 3.5, epsilon = 1e-10);
    }
}
