pub mod error;
mod initial;
pub mod operation;
mod shape;
mod util;

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Not, Sub};
use std::sync::{mpsc, Arc};
use std::thread;
#[derive(Clone)]
pub struct Matrix<const ROWS: usize, const COLS: usize, T>
where
    T: Copy + Send + Sync + Copy,
{
    data: [[T;COLS];ROWS],
    digits: u8,
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync + Display,
{
    pub fn new(data:[[T;COLS];ROWS]) -> Self {
        let mut digits: u8 = 0;
        Matrix { data, digits }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Display + Copy + Send + Sync,
{
    fn display(&self) -> String {
        if ROWS == 0 || COLS == 0 {
            return "┌┐\n└┘\n".to_string();
        } else if ROWS == 1 {
            let line = util::print_single_line(&self.data, self.digits);
            return line;
        }
        let mut result = String::new();
        for row in 0..ROWS {
            let mut line = String::new();
            let (start_char, end_char) = util::get_boundary_char(row, ROWS);
            line.push_str(&start_char);
            let start = row * COLS;
            let end = (row + 1) * COLS;
            for i in start..end {
                let value = self.data[row][i - start];
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


impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: Copy + Send + Sync,
{
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= COLS || y >= ROWS {
            return None;
        }
        Some(self.data[y][x])
    }

    pub fn height(&self) -> usize {
        ROWS
    }

    pub fn width(&self) -> usize {
        COLS
    }

    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), error::OperationError> {
        if x >= COLS || y >= ROWS {
            return Err(error::OperationError {
                message: "index out of bounds".to_string(),
            });
        }
        self.data[y][x] = value;
        Ok(())
    }
}

impl<const N: usize, T> Matrix<N, N, T>
where
    T: Copy + Display + Default + Add<Output = T> + Send + Sync + From<i8> + Mul<Output = T>,
{
    pub fn det(&self) -> Result<T, error::OperationError> {
        let mut sum = T::default();
        let mut err: Option<error::OperationError> = None;
        let permutation = util::permutation(N)?;
        thread::scope(|scope| {
            let data_arc = Arc::new(self.data);
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
                    Ok(x) => sum = sum + x,
                    Err(e) => err = Some(e),
                }
            }
        });
        err.map_or(Ok(sum), Err)
    }
}

impl<const ROWS: usize, const COLS: usize, T> Debug for Matrix<ROWS, COLS, T>
where
    T: Display + Copy + Send + Sync,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<const ROWS: usize, const COLS: usize, T> Display for Matrix<ROWS, COLS, T>
where
    T: Copy + Display + Send + Sync,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_ok() {
        let m = Matrix::<3, 2, i32>::new([[1, 2], [3, 4], [5, 6]]);
        assert_eq!(m.height(), 3);
        assert_eq!(m.width(), 2);
        assert_eq!(m.data, [[1, 2], [3, 4], [5, 6]]);
    }

    #[test]
    fn test_print_blank_matrix() {
        let m = Matrix::<0, 0, i32>::new([]);
        assert_eq!(format!("{}", m), "┌┐\n└┘\n");
    }

    #[test]
    fn test_print_single_line_matrix() {
        let m = Matrix::<1, 3, i32>::new([[1, 2, 3]]);
        assert_eq!(format!("{}", m), "[1  2  3  ]\n");
    }

    #[test]
    fn test_print_multi_line_matrix() {
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert_eq!(format!("{}", m), "┌1   2   ┐\n└3   4   ┘\n");
    }

    #[test]
    fn test_get() {
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert_eq!(m.get(0, 1), Some(3));
        assert_eq!(m.get(2, 1), None);
    }

    #[test]
    fn test_clone() {
        let m1 = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let m2 = m1.clone();
        assert_eq!(m1.data, m2.data);
    }

    #[test]
    fn test_size() {
        let m = Matrix::<3, 4, i32>::new([[0;4];3]);
        assert_eq!(m.size(), (3, 4));
    }

    #[test]
    fn test_set() {
        let mut m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert!(m.set(1, 1, 5).is_ok());
        assert_eq!(m.data[1][1], 5);
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert!(m.set(2, 1, 5).is_err());
    }

    #[test]
    fn test_det_2x2() {
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert_eq!(m.det().unwrap(), -2);
    }

    #[test]
    fn test_det_3x3() {
        let m = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.det().unwrap(), 0);
    }
}
