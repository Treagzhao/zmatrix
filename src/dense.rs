pub mod error;
mod util;
mod initial;
mod shape;
pub mod column_vector;

use std::ops::{Add, Mul, Not, Sub};
use std::fmt::{Debug, Display, Formatter};
use std::sync::{mpsc, Arc};
use std::thread;

pub struct Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    height: u64,
    width: u64,
    data: Vec<T>,
    digits: u8,
}

impl<T> Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    pub fn new(height: u64, width: u64, vec: Vec<T>) -> Result<Matrix<T>, error::OperationError>
    where
        T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
        f64: From<T>,
    {
        if height * width != vec.len().try_into().unwrap() {
            return Result::Err(error::OperationError { message: "vec length doest not match height  & width".to_string() });
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
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
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
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    pub fn get(&self, x: u64, y: u64) -> Option<T> {
        if (x >= self.width) || (y >= self.height) {
            return None;
        }
        let index: usize = (y * self.width + x) as usize;
        self.data.get(index).cloned()
    }

    pub fn det(&self) -> Result<T, error::OperationError> {
        if self.height != self.width {
            return Result::Err(error::OperationError { message: "height and width should be equal".to_string() });
        }
        let mut sum = T::default();
        let mut err: Option<error::OperationError> = None;
        let permutation = util::permutation(self.height)?;
        thread::scope(|scope| {
            let data_arc = Arc::new(self.data.clone());
            let (sender, receiver) = mpsc::channel
                ::<Result<T, error::OperationError>>();
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

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn size(&self) -> (u64, u64) {
        (self.height, self.width)
    }

    pub fn set(&mut self, x: u64, y: u64, value: T) -> Result<(), error::OperationError> {
        let index: usize = (y * self.width + x) as usize;
        if index >= self.data.len() {
            return Result::Err(error::OperationError { message: "index out of bounds".to_string() });
        }
        self.data[index] = value;
        Result::Ok(())
    }

    pub fn product(&self, target: &Matrix<T>) -> Result<Matrix<T>, error::OperationError> {
        if self.width != target.height {
            return Result::Err(error::OperationError { message: "target height and width do not match".to_string() });
        }
        let mut result: Vec<T> = vec![T::default(); (self.height * target.width) as usize];
        let self_data = Arc::new(self.data.clone());
        let target_data = Arc::new(target.data.clone());
        let mut err: Option<error::OperationError> = None;
        thread::scope(|scope| {
            let (sender, receiver) = mpsc::channel::<Result<util::CalculateResult<T>, error::OperationError>>();
            for row in 0..self.height {
                for col in 0..target.width {
                    let self_data_ = self_data.clone();
                    let target_data_ = target_data.clone();
                    let s = sender.clone();
                    scope.spawn(move || {
                        let res = util::calculate_multi(&self_data_, &target_data_, self.height, target.width, row, col, self.width);
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
                    Err(e) => { err = Option::Some(e) }
                }
            }
        });
        if let Some(e) = err {
            return Result::Err(e);
        }
        let m = Matrix::new(self.height, target.width, result)?;
        Result::Ok(m)
    }

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

impl<T> Debug for Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.display();
        write!(f, "{}", result)
    }
}


impl<T> Display for Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.display();
        write!(f, "{}", result)
    }
}
impl<T> Add for Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn add(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(&self.data, &other.data, self.height, self.width, |a, b| -> T {
            a + b
        }).unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Sub for Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    type Output = Matrix<T>;
    fn sub(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(&self.data, &other.data, self.height, self.width, |a, b| -> T {
            a - b
        }).unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64> + From<i8>,
    f64: From<T>,
{
    type Output = Matrix<T>;

    fn mul(self, other: Self) -> Self::Output {
        if self.data.len() != other.data.len() {
            panic!("only matrix in same height & width could be operated");
        }
        let vec = util::calculate_in_threads(&self.data, &other.data, self.height, self.width, |a, b| -> T {
            a * b
        }).unwrap();
        Matrix::new(self.height, self.width, vec).unwrap()
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
        assert_eq!("[1        2        343      4123123  5        6        ]\n", result);

        let vec: Vec<f32> = vec![1.2, 2.123, 343.1, 4.45123123, 5.5654, 6.0];
        let m = Matrix::new(1, 6, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!("[1.2       2.123     343.1     4.451231  5.5654    6         ]\n", result);
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

        let vec: Vec<f64> = vec![1.12, 2.231, 3.1, 4.123123, 5.123123, 6.43422342323, 7.121, 8.1, 90.0];
        let m = Matrix::new(3, 3, vec).unwrap();
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!(r#"┌1.12           2.231          3.1            ┐
|4.123123       5.123123       6.43422342323  |
└7.121          8.1            90             ┘
"#, result);
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
    fn test_size() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(m.height(), 2);
        assert_eq!(m.width(), 3);
        let (h, w) = m.size();
        assert_eq!(h, 2);
        assert_eq!(w, 3);
    }

    #[test]
    fn test_scale() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m1 = m.scale(2);
        assert_eq!(2, m1.height);
        assert_eq!(3, m1.width);
        assert_eq!(vec![2, 4, 6, 8, 10, 12], m1.data);
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

        let m1: Matrix<i32> = Matrix::new(4, 4, vec![1, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16]).unwrap();
        let result = m1.det().unwrap();
        assert_eq!(0, result);

        let m1 = Matrix::new(5, 5, vec![
            1, 1, 1, 1, 1,
            1, 2, 3, 4, 5,
            1, 3, 6, 10, 15,
            1, 3, 10, 20, 35,
            1, 5, 15, 35, 70,
        ]).unwrap();
        let result = m1.det().unwrap();
        assert_eq!(-18, result);
    }

}