use crate::dense::error::OperationError;
use crate::dense::Matrix;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct ColumnVector<T>
where
    T: Copy + Display + Default + Send + Sync,
{
    data: Vec<T>,
    pub(crate) height: usize,
}

impl<T> ColumnVector<T>
where
    T: Copy + Display + Default + Send + Sync,
{
    pub fn new(data: &Vec<T>) -> ColumnVector<T> {
        ColumnVector {
            data: data.clone(),
            height: data.len(),
        }
    }

    pub fn to_matrix(&self) -> Matrix<T> {
        let vec = self.data.clone();
        Matrix::new(self.height, 1, vec).unwrap()
    }

    pub fn zeros(height: usize) -> ColumnVector<T> {
        let mut data = Vec::with_capacity(height);
        for _ in 0..height {
            data.push(T::default());
        }
        ColumnVector { data, height }
    }

    pub fn get(&self, i: usize) -> Result<T, OperationError> {
        if i >= self.height {
            return Err(OperationError {
                message: "out of index".to_string(),
            });
        }
        Ok(self.data[i])
    }
}

impl<T> Display for ColumnVector<T>
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
        let mut s = String::from("--\n");
        for i in 0..self.data.len() {
            s.push_str(&format!("│{}│\n", self.data[i]));
        }
        s.push_str("--");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_column_vector() {
        let data = vec![1, 2, 3, 4, 5];
        let column_vector = ColumnVector::new(&data);
        assert_eq!(column_vector.data, data);
    }

    #[test]
    fn test_column_vector_display() {
        let data = vec![1, 2, 3, 4, 5];
        let column_vector = ColumnVector::new(&data);
        let s = format!("{}", column_vector);
        println!("{}", s);
        assert_eq!(
            s,
            "--
│1│
│2│
│3│
│4│
│5│
--"
        );
    }

    #[test]
    fn test_column_vector_get() {
        let data = vec![1, 2, 3, 4, 5];
        let column_vector = ColumnVector::new(&data);
        for i in 0..data.len() {
            assert_eq!(column_vector.get(i).unwrap(), data[i]);
        }
    }

    #[test]
    #[should_panic(expected = "out of index")]
    fn test_column_vector_get_error() {
        let data = vec![1, 2, 3, 4, 5];
        let column_vector = ColumnVector::new(&data);
        column_vector.get(5).unwrap();
    }

    #[test]
    fn test_column_vector_zeros() {
        let column_vector = ColumnVector::<f64>::zeros(5);
        assert_eq!(column_vector.data, vec![0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(column_vector.height, 5);
    }

    #[test]
    fn test_clone() {
        let data = vec![1, 2, 3, 4, 5];
        let column_vector = ColumnVector::new(&data);
        let column_vector2 = column_vector.clone();
        assert_eq!(column_vector.data, column_vector2.data);
        assert_eq!(column_vector.height, column_vector2.height);
    }


    #[test]
    fn test_to_matrix_with_non_empty_vector() {
        // Test converting a non-empty ColumnVector to Matrix
        let data = vec![1, 2, 3];
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        assert_eq!(matrix.height(), 3);
        assert_eq!(matrix.width(), 1);
        assert_eq!(matrix.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_to_matrix_with_single_element() {
        // Test converting a single-element ColumnVector to Matrix
        let data = vec![42];
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        assert_eq!(matrix.height(), 1);
        assert_eq!(matrix.width(), 1);
        assert_eq!(matrix.data, vec![42]);
    }

    #[test]
    fn test_to_matrix_with_empty_vector() {
        // Test converting an empty ColumnVector to Matrix
        let data: Vec<i32> = vec![];
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        assert_eq!(matrix.height(), 0);
        assert_eq!(matrix.width(), 1);
        assert!(matrix.data.is_empty());
    }

    #[test]
    fn test_to_matrix_verify_dimensions() {
        // Verify the resulting matrix dimensions are correct
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        assert_eq!(matrix.height(), column_vector.height);
        assert_eq!(matrix.width(), 1);
    }

    #[test]
    fn test_to_matrix_data_integrity() {
        // Verify the data is correctly transferred to matrix
        let data = vec![10, 20, 30, 40, 50];
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        for i in 0..column_vector.height {
            assert_eq!(matrix.get(0,i).unwrap(), column_vector.get(i).unwrap());
        }
    }

    #[test]
    fn test_to_matrix_with_large_vector() {
        // Test with a large vector to check performance/stability
        let data: Vec<_> = (0..1000).collect();
        let column_vector = ColumnVector::new(&data);
        let matrix = column_vector.to_matrix();

        assert_eq!(matrix.height(), 1000);
        assert_eq!(matrix.width(), 1);
        assert_eq!(matrix.get(0,999).unwrap(), 999);
    }
}
