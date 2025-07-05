pub mod column_vector;
pub mod error;
mod initial;
mod operation;
mod shape;
mod util;

use crate::dense::column_vector::ColumnVector;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Not, Sub};
use std::sync::{mpsc, Arc};
use std::thread;

pub struct Matrix<T>
where
    T: Copy + Send + Sync,
{
    height: usize,
    width: usize,
    data: Vec<T>,
    digits: u8,
}

impl<T> Matrix<T>
where
    T: Copy + Send + Sync + Display,
{
    pub fn new(
        height: usize,
        width: usize,
        vec: Vec<T>,
    ) -> Result<Matrix<T>, error::OperationError> {
        if height * width != vec.len().try_into().unwrap() {
            return Result::Err(error::OperationError {
                message: "vec length doest not match height  & width".to_string(),
            });
        }
        let mut digits: u8 = 0;
        for value in vec.iter() {
            let f = *value;
            let d = f.to_string().len() as u8;
            if d > digits {
                digits = d
            }
        }
        Result::Ok(Matrix {
            height,
            width,
            data: vec,
            digits,
        })
    }
}

impl<T> Matrix<T>
where
    T: Display + Copy + Send + Sync,
{
    fn display(&self) -> String {
        if self.data.len() == 0 {
            return "┌┐\n└┘\n".to_string();
        } else if self.height == 1 {
            let line = util::print_single_line(&self.data, self.digits);
            return line;
        }
        let mut result = String::new();
        for row in 0..self.height {
            let mut line = String::new();
            let (start_char, end_char) = util::get_boundary_char(row, self.height);
            line.push_str(&start_char);
            let start = row * self.width;
            let end = (row + 1) * self.width;
            for i in start..end {
                let value = self.data[i as usize];
                let str = format!("{:<digit$}", value, digit = (self.digits + 2) as usize);
                line.push_str(&str);
            }
            line.push_str(&end_char);
            line.push('\n');
            result.push_str(&line);
        }
        result
    }
}

impl<T> Clone for Matrix<T>
where
    T: Copy + Send + Sync + Display,
{
    fn clone(&self) -> Self {
        let vec = self.data.clone();
        let height = self.height;
        let width = self.width;
        Matrix::new(height, width, vec).unwrap()
    }
}

impl<T> Matrix<T>
where
    T: Copy + Send + Sync,
{
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if (x >= self.width) || (y >= self.height) {
            return None;
        }
        let index: usize = (y * self.width + x) as usize;
        self.data.get(index).cloned()
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), error::OperationError> {
        let index: usize = (y * self.width + x) as usize;
        if index >= self.data.len() {
            return Result::Err(error::OperationError {
                message: "index out of bounds".to_string(),
            });
        }
        self.data[index] = value;
        Result::Ok(())
    }
}

impl<T> Matrix<T>
where
    T: Copy + Display + Default + Add<Output = T> + Send + Sync + From<i8> + Mul<Output = T>,
{
    pub fn det(&self) -> Result<T, error::OperationError> {
        if self.height != self.width {
            return Result::Err(error::OperationError {
                message: "height and width should be equal".to_string(),
            });
        }
        let mut sum = T::default();
        let mut err: Option<error::OperationError> = None;
        let permutation = util::permutation(self.height)?;
        thread::scope(|scope| {
            let data_arc = Arc::new(self.data.clone());
            let (sender, receiver) = mpsc::channel::<Result<T, error::OperationError>>();
            for perm in permutation {
                let data = data_arc.clone();
                let s = sender.clone();
                scope.spawn(move || {
                    let res = util::determinant_in_one_permutation(&data, &perm);
                    s.send(res).unwrap();
                });
            }
            drop(sender);
            for res in receiver {
                match res {
                    Ok(x) => {
                        sum = sum + x;
                    }
                    Err(e) => err = Some(e),
                }
            }
        });
        if let Some(e) = err {
            return Result::Err(e);
        }
        Result::Ok(sum)
    }
}

impl<T> Debug for Matrix<T>
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.display();
        write!(f, "{}", result)
    }
}

impl<T> Display for Matrix<T>
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.display();
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_ok() {
        let result = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        assert!(match result {
            Result::Ok(_) => true,
            Err(_) => false,
        });
        if let Result::Ok(m) = result {
            assert_eq!(m.height, 3);
            assert_eq!(m.width, 2);
            assert_eq!(m.data, vec![1, 2, 3, 4, 5, 6]);
        }
        let result = Matrix::new(3, 2, vec![11, 21, 33, 4, 5, 6]);
        let result = Matrix::new(3, 2, vec![111, 21, 33, 4, 5, 6]);
    }
    #[test]
    fn new_not_ok() {
        let result = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert!(match result {
            Result::Ok(_) => false,
            Err(_) => true,
        })
    }

    #[test]
    fn test_print_blank_matrix() {
        let vec: Vec<i32> = Vec::new();
        let m = Matrix::new(0, 0, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!("┌┐\n└┘\n", result);
        let result = format!("{:?}", m);
        println!("{}", result.clone());
        assert_eq!("┌┐\n└┘\n", result);
    }

    #[test]
    fn test_print_single_line_matrix() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let m = Matrix::new(1, 6, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!("[1  2  3  4  5  6  ]\n", result);

        let result = format!("{:?}", m);
        println!("{}", result.clone());
        assert_eq!("[1  2  3  4  5  6  ]\n", result);

        let vec: Vec<i32> = vec![1, 2, 343, 4123123, 5, 6];
        let m = Matrix::new(1, 6, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!(
            "[1        2        343      4123123  5        6        ]\n",
            result
        );

        let vec: Vec<f32> = vec![1.2, 2.123, 343.1, 4.45123123, 5.5654, 6.0];
        let m = Matrix::new(1, 6, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!(
            "[1.2       2.123     343.1     4.451231  5.5654    6         ]\n",
            result
        );
    }

    #[test]
    fn test_print_multi_line_matrix() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 90];
        let m = Matrix::new(3, 3, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!("┌1   2   3   ┐\n|4   5   6   |\n└7   8   90  ┘\n", result);
        let result = format!("{:?}", m);
        println!("{}", result.clone());
        assert_eq!("┌1   2   3   ┐\n|4   5   6   |\n└7   8   90  ┘\n", result);

        let vec: Vec<f64> = vec![
            1.12,
            2.231,
            3.1,
            4.123123,
            5.123123,
            6.43422342323,
            7.121,
            8.1,
            90.0,
        ];
        let m = Matrix::new(3, 3, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!(
            r#"┌1.12           2.231          3.1            ┐
|4.123123       5.123123       6.43422342323  |
└7.121          8.1            90             ┘
"#,
            result
        );
    }

    #[test]
    fn test_get() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let v = m.get(0, 1);
        assert!(match v {
            Some(_) => true,
            None => false,
        });
        if let Some(x) = v {
            assert_eq!(3, x);
        }
        let v = m.get(3, 2);
        assert!(match v {
            Some(_) => false,
            None => true,
        });

        let m = Matrix::new(3, 1, vec![1, 2, 3]).unwrap();
        let v = m.get(1, 0);
        assert!(v.is_none());

        let m = Matrix::new(1, 3, vec![1, 2, 3]).unwrap();
        let v = m.get(0, 1);
        assert!(v.is_none());
    }

    #[test]
    fn test_clone() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m1 = m.clone();
        assert_eq!(m1.height, m.height);
        assert_eq!(m1.width, m.width);
        assert_eq!(m1.data, m.data);
        assert_eq!(m1.digits, m.digits);
    }

    #[test]
    fn test_size() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(m.height(), 2);
        assert_eq!(m.width(), 3);
        let (h, w) = m.size();
        assert_eq!(h, 2);
        assert_eq!(w, 3);
    }

    #[test]
    fn test_set() {
        let mut m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let result = m.set(1, 1, 12);
        assert!(result.is_ok());
        let v = m.get(1, 1);
        assert!(v.is_some());
        if let Some(x) = v {
            assert_eq!(x, 12);
        }
    }

    #[test]
    fn test_determinant_ok() {
        let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let result = m1.det();
        assert!(result.is_ok());
        if let Ok(x) = result {
            println!("{}", x);
        }

        let m1 = Matrix::new(2, 2, vec![2, 3, 4, 5]).unwrap();
        let result = m1.det().unwrap();
        assert_eq!(-2, result);

        let m1 = Matrix::new(4, 4, vec![1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4]).unwrap();
        let result = m1.det().unwrap();
        assert_eq!(24, result);

        let m1: Matrix<i32> = Matrix::new(
            4,
            4,
            vec![1, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16],
        )
        .unwrap();
        let result = m1.det().unwrap();
        assert_eq!(0, result);

        let m1 = Matrix::new(
            5,
            5,
            vec![
                1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 1, 3, 6, 10, 15, 1, 3, 10, 20, 35, 1, 5, 15, 35, 70,
            ],
        )
        .unwrap();
        let result = m1.det().unwrap();
        assert_eq!(-18, result);
    }

    #[test]
    #[should_panic]
    fn test_add_column_vector_panic() {
        let m1: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let v1: ColumnVector<f64> = ColumnVector::new(&vec![6.0, 7.0, 8.0]);
        let result = m1 + v1;
    }

    #[test]
    fn test_add_column_vector() {
        let m1: Matrix<f64> = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let v1: ColumnVector<f64> = ColumnVector::new(&vec![6.0, 7.0, 8.0]);
        let result = m1 + v1;

        assert_eq!((2, 3), result.size());
        assert_eq!(vec![7.0, 9.0, 11.0, 10.0, 12.0, 14.0], result.data);
    }
}
