use std::convert::From;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::sync::{mpsc, Arc};
use std::thread;
use crate::dense::error;

pub struct CalculateResult<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64>,
    f64: From<T>,
{
    pub x: u64,
    pub y: u64,
    pub value: T,
}

pub fn convert_to_f64<T>(value: T) -> Result<f64, error::OperationError>
where
    T: std::convert::Into<f64>,
    f64: From<T>,
{
    let result = value.try_into();
    match result {
        Ok(value) => Ok(value),
        Err(_) => {
            return Result::Err(error::OperationError { message: "value error".to_string() });
        }
    }
}

pub fn print_single_line<T>(data: &Vec<T>, digits: u8) -> String
where
    T: Display,
{
    let mut line = String::from("[");
    data.iter().for_each(|item| {
        let str = format!("{:<digit$}", *item, digit = (digits + 2) as usize);
        line.push_str(&str);
    });
    line.push_str("]\n");
    line.to_string()
}

pub fn get_boundary_char(row: u64, height: u64) -> (String, String) {
    if height == 1 {
        return ("[".to_string(), "]".to_string());
    } else if row == 0 {
        return ("┌".to_string(), "┐".to_string());
    } else if row == height - 1 {
        return ("└".to_string(), "┘".to_string());
    } else {
        return ("|".to_string(), "|".to_string());
    }
}

pub fn calculate_in_threads<'a, T, F>(a_data: &'a Vec<T>, b_data: &'a Vec<T>, height: u64, width: u64, func: F) -> Result<Vec<T>, error::OperationError>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64>,
    F: Fn(T, T) -> T + Send + Sync + Copy,
    f64: From<T>,
{
    if a_data.len() != b_data.len() {
        return Result::Err(error::OperationError { message: "the length of two data should be equal".to_string() });
    }
    if a_data.len() != (height * width) as usize {
        return Result::Err(error::OperationError { message: "the length of data should be equal with height times width".to_string() });
    }
    let length = a_data.len();
    let mut vec: Vec<T> = vec![T::default(); length];
    thread::scope(|scope| {
        let a_vec: Arc<&'a Vec<T>> = Arc::new(a_data);
        let b_vec: Arc<&'a Vec<T>> = Arc::new(b_data);
        let (sender, receiver) = mpsc::channel::<CalculateResult<T>>();
        for row in 0..height {
            for col in 0..width {
                let self_data = a_vec.clone();
                let other_data = b_vec.clone();
                let w = width;
                let s = sender.clone();
                scope.spawn(move || {
                    let index = (row * w + col) as usize;
                    let value = func(self_data[index], other_data[index]);
                    s.send(CalculateResult {
                        x: col,
                        y: row,
                        value,
                    })
                });
            }
        }
        drop(sender);
        for rv in receiver {
            let index = (rv.y * width + rv.x) as usize;
            vec[index] = rv.value;
        }
    });
    Result::Ok(vec)
}

pub fn calculate_multi<T>(a: &Vec<T>, b: &Vec<T>, _a_height: u64, b_width: u64, cur_row: u64, cur_col: u64, count: u64) -> Result<CalculateResult<T>, error::OperationError>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display + Default + Send + Sync + TryInto<f64>,
    f64: From<T>,
{
    let mut sum: T = T::default();
    let start_a: usize = (cur_row * count) as usize;
    for i in 0..count {
        let index_a = start_a + i as usize;
        let index_b = (i * b_width + cur_col) as usize;
        if index_a >= a.len() || index_b >= b.len() {
            return Result::Err(error::OperationError { message: "index error".to_string() });
        }
        let value = a[index_a] * b[index_b];
        sum = sum + value;
    }
    Result::Ok(CalculateResult { x: cur_col, y: cur_row, value: sum })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_convert_to_f64() {
        let a: i32 = 10;
        let a_r = convert_to_f64(a);
        assert!(match a_r {
            Ok(_) => true,
            Err(_) => false,
        });
        if let Ok(x) = a_r {
            assert_eq!(x, a as f64);
        }
        let b: i16 = 32343;
        let b_r = convert_to_f64(b);
        assert!(match a_r {
            Ok(_) => true,
            Err(_) => false,
        });
        if let Ok(x) = b_r {
            assert_eq!(x, b as f64);
        }
    }

    #[test]
    fn test_print_single_line() {
        let a: Vec<i32> = vec![1, 2, 3, 4, 15];
        let result = print_single_line(&a, 1);
        println!("{}", result.clone());
        assert_eq!("[1  2  3  4  15 ]\n", result);
    }

    #[test]
    fn test_get_boundary_char() {
        let (start, end) = get_boundary_char(2, 1);
        assert_eq!("[", start);
        assert_eq!("]", end);
        let (start, end) = get_boundary_char(0, 3);
        assert_eq!("┌", start);
        assert_eq!("┐", end);
        let (start, end) = get_boundary_char(1, 3);
        assert_eq!("|", start);
        assert_eq!("|", end);
        let (start, end) = get_boundary_char(2, 3);
        assert_eq!("└", start);
        assert_eq!("┘", end);
    }

    #[test]
    fn test_calculate_in_threads() {
        let a: Vec<i32> = vec![1, 2, 3, 4, 15, 18];
        let b: Vec<i32> = vec![2, 4, 6, 8, 10, 22];
        let result = calculate_in_threads(&a, &b, 2, 3, |a, b| -> i32{
            return a + b;
        });
        assert!(result.is_ok());
        if let Ok(vec) = result {
            assert_eq!(vec, vec![3, 6, 9, 12, 25, 40]);
        }
        let a: Vec<i32> = vec![1, 2, 3, 4, 15];
        let b: Vec<i32> = vec![2, 4, 6, 8, 10, 22];
        let result = calculate_in_threads(&a, &b, 2, 3, |a, b| -> i32{
            return a + b;
        });
        assert_eq!("the length of two data should be equal", result.err().unwrap().message);
        let a: Vec<i32> = vec![1, 2, 3, 4, 1, 33];
        let b: Vec<i32> = vec![2, 4, 6, 8, 10, 22];
        let result = calculate_in_threads(&a, &b, 4, 3, |a, b| -> i32{
            return a + b;
        });
        assert_eq!("the length of data should be equal with height times width", result.err().unwrap().message);
    }

    #[test]
    fn test_calculate_multi() {
        let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let b: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let result = calculate_multi(&a, &b, 2, 4, 0, 1, 3);
        assert!(result.is_ok());
        if let Ok(res) = result {
            assert_eq!(res.x, 1);
            assert_eq!(res.y, 0);
            assert_eq!(res.value, 44);
        }
        let result = calculate_multi(&a, &b, 2, 4, 0, 11, 3);
        assert_eq!("index error", result.err().unwrap().message);
    }
}