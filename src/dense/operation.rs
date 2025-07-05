use crate::dense::column_vector::ColumnVector;
use crate::dense::{util, Matrix};
use std::fmt::Display;
use std::ops::{Add, Mul, Neg, Sub};

impl<T> Add for Matrix<T>
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
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Display
        + Neg<Output=T>
        + Default
        + Send
        + Sync
        + TryInto<f64>
        + From<i8>,
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

#[cfg(test)]
mod test {
    use super::*;
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
}
