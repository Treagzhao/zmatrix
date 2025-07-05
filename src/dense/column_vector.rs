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

impl<T> Matrix<T>
where
    T: Copy + Display + Default + Send + Sync,
{
    pub fn to_column_vector(&self) -> Result<ColumnVector<T>, OperationError> {
        if self.width != 1 {
            return Err(OperationError {
                message: "only 1 column matrix could be transferred to column vector".to_string(),
            });
        }
        let data = self.data.clone();
        Ok(ColumnVector::new(&data))
    }
}

impl<T> Add<ColumnVector<T>> for ColumnVector<T>
where
    T: Copy + Display + Default + Send + Sync + Add<Output = T>,
{
    type Output = ColumnVector<T>;

    fn add(self, rhs: ColumnVector<T>) -> Self::Output {
        if self.height != rhs.height {
            panic!("column vector size not match");
        }
        let mut vec = Vec::with_capacity(self.height);
        for i in 0..self.height {
            vec.push(self.data[i] + rhs.data[i]);
        }
        ColumnVector::new(&vec)
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
            assert_eq!(matrix.get(0, i).unwrap(), column_vector.get(i).unwrap());
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
        assert_eq!(matrix.get(0, 999).unwrap(), 999);
    }

    #[test]
    fn test_to_column_vector_success() {
        // Test converting a valid 1-column matrix to column vector
        let data = vec![1, 2, 3];
        let matrix = Matrix::new(3, 1, data).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_ok());
        let column_vector = result.unwrap();
        assert_eq!(column_vector.height, 3);
        assert_eq!(column_vector.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_to_column_vector_single_element() {
        // Test converting a 1x1 matrix
        let data = vec![42];
        let matrix = Matrix::new(1, 1, data).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_ok());
        let column_vector = result.unwrap();
        assert_eq!(column_vector.height, 1);
        assert_eq!(column_vector.data, vec![42]);
    }

    #[test]
    fn test_to_column_vector_empty_matrix() {
        // Test converting an empty matrix (0x1)
        let data: Vec<i32> = vec![];
        let matrix = Matrix::new(0, 1, data).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_ok());
        let column_vector = result.unwrap();
        assert_eq!(column_vector.height, 0);
        assert!(column_vector.data.is_empty());
    }

    #[test]
    fn test_to_column_vector_failure_multi_column() {
        // Test converting a multi-column matrix should fail
        let data = vec![1, 2, 3, 4];
        let matrix = Matrix::new(2, 2, data).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "only 1 column matrix could be transferred to column vector"
        );
    }

    #[test]
    fn test_to_column_vector_data_integrity() {
        // Verify the data is correctly transferred
        let data = vec![10.5, 20.25, 30.75];
        let matrix = Matrix::new(3, 1, data.clone()).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_ok());
        let column_vector = result.unwrap();
        assert_eq!(column_vector.data, data);
    }

    #[test]
    fn test_to_column_vector_large_matrix() {
        // Test with large matrix for performance/stability check
        let data: Vec<_> = (0..1000).map(|x| x as f32).collect();
        let matrix = Matrix::new(1000, 1, data.clone()).unwrap();
        let result = matrix.to_column_vector();

        assert!(result.is_ok());
        let column_vector = result.unwrap();
        assert_eq!(column_vector.height, 1000);
        assert_eq!(column_vector.data[999], 999.0);
    }


    #[test]
    fn test_add_column_vectors_success() {
        // Test normal case: add two vectors of same size
        let vec1 = ColumnVector::new(&vec![1, 2, 3]);
        let vec2 = ColumnVector::new(&vec![4, 5, 6]);
        let result = vec1.add(vec2);

        assert_eq!(result.data, vec![5, 7, 9]);
        assert_eq!(result.height, 3);
    }

    #[test]
    #[should_panic(expected = "column vector size not match")]
    fn test_add_column_vectors_size_mismatch() {
        // Test error case: add vectors of different sizes
        let vec1 = ColumnVector::new(&vec![1, 2, 3]);
        let vec2 = ColumnVector::new(&vec![4, 5]);
        vec1.add(vec2);
    }

    #[test]
    fn test_add_empty_vectors() {
        // Test edge case: add two empty vectors
        let vec1 = ColumnVector::<i32>::zeros(0);
        let vec2 = ColumnVector::<i32>::zeros(0);
        let result = vec1.add(vec2);

        assert_eq!(result.data, vec![]);
        assert_eq!(result.height, 0);
    }

    #[test]
    fn test_add_large_vectors() {
        // Test performance case: add large vectors
        let data1: Vec<_> = (0..1000).collect();
        let data2: Vec<_> = (0..1000).rev().collect();
        let vec1 = ColumnVector::new(&data1);
        let vec2 = ColumnVector::new(&data2);
        let result = vec1.add(vec2);

        assert_eq!(result.height, 1000);
        for i in 0..result.height {
            assert_eq!(result.data[i], 999);
        }
    }

    #[test]
    fn test_add_vectors_with_zeros() {
        // Test special case: add vectors containing zeros
        let vec1 = ColumnVector::new(&vec![1, 0, 3]);
        let vec2 = ColumnVector::new(&vec![0, 5, 0]);
        let result = vec1.add(vec2);

        assert_eq!(result.data, vec![1, 5, 3]);
    }

    #[test]
    fn test_add_vectors_with_f64_values() {
        // Test float precision case
        let vec1 = ColumnVector::new(&vec![1.5, 2.25]);
        let vec2 = ColumnVector::new(&vec![0.5, 0.75]);
        let result = vec1.add(vec2);

        assert_eq!(result.data, vec![2.0, 3.0]);
    }

    #[test]
    fn test_add_vectors_verify_immutability() {
        // Verify original vectors are not modified
        let data1 = vec![1, 2];
        let data2 = vec![3, 4];
        let vec1 = ColumnVector::new(&data1);
        let vec2 = ColumnVector::new(&data2);
        let _ = vec1.clone().add(vec2.clone());

        assert_eq!(vec1.data, data1);
        assert_eq!(vec2.data, data2);
    }
}
