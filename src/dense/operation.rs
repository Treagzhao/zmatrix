use crate::dense::column_vector::ColumnVector;
use crate::dense::{error, util, Matrix};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{mpsc, Arc};
use std::thread;

impl<T> Add for Matrix<T>
where
    T: Copy + Add<Output = T> + Display + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn add(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(
            &self.data,
            &other.data,
            self.height,
            self.width,
            |a, b| -> T { a + b },
        )
        .unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Sub for Matrix<T>
where
    T: Copy + Sub<Output = T> + Display + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;
    fn sub(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(
            &self.data,
            &other.data,
            self.height,
            self.width,
            |a, b| -> T { a - b },
        )
        .unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + Display + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn mul(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(
            &self.data,
            &other.data,
            self.height,
            self.width,
            |a, b| -> T { a * b },
        )
        .unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Add<ColumnVector<T>> for Matrix<T>
where
    T: Copy + Add<Output = T> + Display + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: ColumnVector<T>) -> Self::Output {
        if self.width != rhs.height {
            panic!("only matrix in same height could be operated");
        }
        let mut vec: Vec<T> = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let v = self.data[row * self.width + col].clone() + rhs.get(col).unwrap();
                vec.push(v);
            }
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Neg for Matrix<T>
where
    T: Copy + Neg<Output = T> + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        let mut vec = self.data.clone();
        for i in 0..vec.len() {
            vec[i] = -vec[i];
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Matrix<T>
where
    T: Copy + Default + Send + Sync + TryInto<f64> + From<i8> + Display,
    f64: From<T>,
{
    pub fn exp(&self) -> Matrix<f64> {
        let mut vec: Vec<f64> = Vec::new();
        for i in 0..self.data.len() {
            vec.push(f64::from(self.data[i]).exp());
        }
        Matrix::<f64>::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Add<T> for Matrix<T>
where
    T: Copy + Add<Output = T> + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut vec = self.data.clone();
        for i in 0..vec.len() {
            vec[i] = vec[i] + rhs;
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Sub<T> for Matrix<T>
where
    T: Copy + Sub<Output = T> + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let mut vec = self.data.clone();
        for i in 0..vec.len() {
            vec[i] = vec[i] - rhs;
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Copy + Mul<Output = T> + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut vec = self.data.clone();
        for i in 0..vec.len() {
            vec[i] = vec[i] * rhs;
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Div<T> for Matrix<T>
where
    T: Copy + Div<Output = T> + Default + Send + Sync + TryInto<f64> + Display,
    f64: From<T>,
{
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output {
        let mut vec = self.data.clone();
        for i in 0..vec.len() {
            vec[i] = vec[i] / rhs;
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}
//
// impl<T> Sub<Matrix<T>> for T
// where
//     T: Copy + Sub<Output = T> + Display + Send + Sync,
// {
//     type Output = Matrix<T>;
//
//     fn sub(self, rhs: Matrix<T>) -> Self::Output {
//         let mut vec = rhs.data.clone();
//         for i in 0..vec.len() {
//             vec[i] = self - vec[i];
//         }
//         Matrix::new(rhs.height, rhs.width, vec).unwrap()
//     }
// }

impl<T> Matrix<T>
where
    T: Default + Display + Send + Sync + Copy + Add<Output=T> + Mul<Output=T>,
{
    pub fn product(&self, target: &Matrix<T>) -> Result<Matrix<T>, error::OperationError> {
        if self.width != target.height {
            return Result::Err(error::OperationError {
                message: "target height and width do not match".to_string(),
            });
        }
        let mut result: Vec<T> = vec![T::default(); (self.height * target.width) as usize];
        let self_data = Arc::new(self.data.clone());
        let target_data = Arc::new(target.data.clone());
        let mut err: Option<error::OperationError> = None;
        thread::scope(|scope| {
            let (sender, receiver) =
                mpsc::channel::<Result<util::CalculateResult<T>, error::OperationError>>();
            for row in 0..self.height {
                for col in 0..target.width {
                    let self_data_ = self_data.clone();
                    let target_data_ = target_data.clone();
                    let s = sender.clone();
                    scope.spawn(move || {
                        let res = util::calculate_multi(
                            &self_data_,
                            &target_data_,
                            self.height,
                            target.width,
                            row,
                            col,
                            self.width,
                        );
                        s.send(res).unwrap();
                    });
                }
            }
            drop(sender);
            for rc in receiver {
                match rc {
                    Ok(res) => {
                        let index: usize = (res.y * target.width + res.x) as usize;
                        result[index] = res.value;
                    }
                    Err(e) => err = Option::Some(e),
                }
            }
        });
        if let Some(e) = err {
            return Result::Err(e);
        }
        let m = Matrix::new(self.height, target.width, result)?;
        Result::Ok(m)
    }
}

impl<T> Matrix<T>
where T:Display + Default + Send + Sync + Mul<Output=T> + Copy{
    pub fn scale(&self, scalar: T) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = (row * self.width + col) as usize;
                let value = self.data[index] * scalar;
                vec.push(value);
            }
        }
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use std::panic;
    #[test]
    fn test_add_ok() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 3, vec![12, 22, 33, 44, 51, 56]).unwrap();
        let m3 = m1 + m2;
        assert_eq!(2, m3.height);
        assert_eq!(3, m3.width);
        println!("{}", m3);
        assert_eq!(vec![13, 24, 36, 48, 56, 62], m3.data);
    }

    #[test]
    #[should_panic]
    fn test_add_not_ok() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 2, vec![12, 22, 33, 44]).unwrap();
        let m3 = m1 + m2;
        println!("{}", m3);
    }

    #[test]
    fn test_sub() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 3, vec![12, 22, 33, 44, 51, 56]).unwrap();
        let m3 = m2 - m1;
        println!("{}", m3);
        assert_eq!(2, m3.height);
        assert_eq!(3, m3.width);
        assert_eq!(vec![11, 20, 30, 40, 46, 50], m3.data);
    }

    #[test]
    #[should_panic]
    fn test_sub_not_ok() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 2, vec![12, 22, 33, 44]).unwrap();
        let m3 = m2 - m1;
        println!("{}", m3);
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 3, vec![2, 3, 4, 5, 6, 7]).unwrap();
        let m3 = m1 * m2;
        assert_eq!(2, m3.height);
        assert_eq!(3, m3.width);
        println!("{}", m3);
        assert_eq!(vec![2, 6, 12, 20, 30, 42], m3.data);
    }

    #[test]
    #[should_panic]
    fn test_mul_not_ok() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(2, 2, vec![2, 3, 4, 5]).unwrap();
        let m3 = m1 * m2;
        println!("{}", m3);
    }

    #[test]
    fn test_matrix_product() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = Matrix::new(3, 4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
        let result = m1.product(&m2);
        assert!(result.is_ok());

        if let Ok(m3) = result {
            println!("{}", m3);
            assert_eq!(2, m3.height);
            assert_eq!(4, m3.width);
            assert_eq!(vec![38, 44, 50, 56, 83, 98, 113, 128], m3.data);
        }
    }

    #[test]
    fn test_neg() {
        let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = -m1;
        assert_eq!(2, m2.height);
        assert_eq!(3, m2.width);
        assert_eq!(vec![-1, -2, -3, -4, -5, -6], m2.data);
    }

    #[test]
    fn test_exp_positive_values() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 0.5, 3.0]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 2);
        assert_eq!(result.width, 2);
        assert_relative_eq!(result.data[0], 1.0f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 2.0f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 0.5f64.exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[3], 3.0f64.exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_negative_values() {
        let matrix = Matrix::new(2, 2, vec![-1.0, -2.0, -0.5, -3.0]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 2);
        assert_eq!(result.width, 2);
        assert_relative_eq!(result.data[0], (-1.0f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], (-2.0f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[2], (-0.5f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[3], (-3.0f64).exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_zero_values() {
        let matrix = Matrix::new(2, 2, vec![0.0, 0.0, 0.0, 0.0]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 2);
        assert_eq!(result.width, 2);
        for value in result.data {
            assert_eq!(value, 1.0); // e^0 = 1
        }
    }

    #[test]
    fn test_exp_large_values() {
        let matrix = Matrix::new(1, 2, vec![10.0, 20.0]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 1);
        assert_eq!(result.width, 2);
        assert_relative_eq!(result.data[0], 10.0f64.exp(), epsilon = 1e10); // Large epsilon for big numbers
        assert_relative_eq!(result.data[1], 20.0f64.exp(), epsilon = 1e10);
    }

    #[test]
    fn test_exp_small_values() {
        let matrix = Matrix::new(1, 2, vec![1e-10, -1e-10]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 1);
        assert_eq!(result.width, 2);
        assert_relative_eq!(result.data[0], (1e-10f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(result.data[1], (-1e-10f64).exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_single_element_matrix() {
        let matrix = Matrix::new(1, 1, vec![2.0]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 1);
        assert_eq!(result.width, 1);
        assert_relative_eq!(result.data[0], 2.0f64.exp(), epsilon = 1e-10);
    }

    #[test]
    fn test_exp_preserves_dimensions() {
        let matrix = Matrix::new(3, 4, vec![1.0; 12]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.height, 3);
        assert_eq!(result.width, 4);
        assert_eq!(result.data.len(), 12);
    }

    #[test]
    fn test_exp_with_nan() {
        let matrix = Matrix::new(1, 1, vec![f64::NAN]).unwrap();
        let result = matrix.exp();

        assert!(result.data[0].is_nan());
    }

    #[test]
    fn test_exp_with_infinity() {
        let matrix = Matrix::new(1, 2, vec![f64::INFINITY, f64::NEG_INFINITY]).unwrap();
        let result = matrix.exp();

        assert_eq!(result.data[0], f64::INFINITY);
        assert_eq!(result.data[1], 0.0);
    }

    #[test]
    fn test_add_scalar() {
        // Test basic addition with scalar
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let result = m + 5;
        assert_eq!(result.data, vec![6, 7, 8, 9]);
        assert_eq!(result.height, 2);
        assert_eq!(result.width, 2);

        // Test with negative scalar
        let m = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
        let result = m + (-5);
        assert_eq!(result.data, vec![5, 15, 25, 35]);
    }

    #[test]
    fn test_add_scalar_float() {
        // Test with floating point numbers
        let m = Matrix::new(1, 3, vec![1.5, 2.5, 3.5]).unwrap();
        let result = m + 1.5;
        assert_relative_eq!(result.data[0], 3.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 4.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sub_scalar() {
        // Test basic subtraction with scalar
        let m = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
        let result = m - 5;
        assert_eq!(result.data, vec![5, 15, 25, 35]);
        let m = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
        // Test with negative scalar
        let result = m - (-5);
        assert_eq!(result.data, vec![15, 25, 35, 45]);
    }

    #[test]
    fn test_mul_scalar() {
        // Test basic multiplication with scalar
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let result = m * 5;
        assert_eq!(result.data, vec![5, 10, 15, 20]);

        // Test with zero
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let result = m * 0;
        assert_eq!(result.data, vec![0, 0, 0, 0]);

        // Test with negative scalar
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let result = m * -1;
        assert_eq!(result.data, vec![-1, -2, -3, -4]);
    }

    #[test]
    fn test_div_scalar() {
        // Test basic division with scalar
        let m = Matrix::new(2, 2, vec![10.0, 20.0, 30.0, 40.0]).unwrap();
        let result = m / 2.0;
        assert_relative_eq!(result.data[0], 5.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 10.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 15.0, epsilon = 1e-10);
        assert_relative_eq!(result.data[3], 20.0, epsilon = 1e-10);

        // Test division by zero
        let m = Matrix::new(1, 2, vec![1.0, 2.0]).unwrap();
        let result = panic::catch_unwind(|| m / 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_scalar_sub_matrix() {
        // Test scalar minus matrix
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let result = 10 - m;
        assert_eq!(result.data, vec![9, 8, 7, 6]);

        // Test with floating point
        let m = Matrix::new(1, 3, vec![1.5, 2.5, 3.5]).unwrap();
        let result = 10.0 - m;
        assert_relative_eq!(result.data[0], 8.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[1], 7.5, epsilon = 1e-10);
        assert_relative_eq!(result.data[2], 6.5, epsilon = 1e-10);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty matrix
        let m: Matrix<i32> = Matrix::new(0, 0, vec![]).unwrap();
        let result = m + 10;
        assert_eq!(result.data, vec![]);

        // Test single element matrix
        let m = Matrix::new(1, 1, vec![5]).unwrap();
        let result = m * 2;
        assert_eq!(result.data, vec![10]);

        // Test large values
        let m = Matrix::new(1, 2, vec![i32::MAX, i32::MIN]).unwrap();
        let result = panic::catch_unwind(|| m + 1);
        assert!(result.is_err()); // Overflow check
    }

    #[test]
    fn test_scale() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = m.scale(2);
        assert_eq!(2, m1.height);
        assert_eq!(3, m1.width);
        assert_eq!(vec![2, 4, 6, 8, 10, 12], m1.data);
    }
}
