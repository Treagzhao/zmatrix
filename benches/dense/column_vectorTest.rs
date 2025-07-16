#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_returns_correct_value() {
        // Test normal case with non-empty vector
        let data = vec![1, 2, 3];
        let vector = ColumnVector::new(&data);
        assert_eq!(vector.height(), 3);
    }

    #[test]
    fn test_height_with_empty_vector() {
        // Test edge case with empty vector
        let data: Vec<i32> = vec![];
        let vector = ColumnVector::new(&data);
        assert_eq!(vector.height(), 0);
    }

    #[test]
    fn test_height_with_single_element() {
        // Test edge case with single element
        let data = vec![42];
        let vector = ColumnVector::new(&data);
        assert_eq!(vector.height(), 1);
    }

    #[test]
    fn test_height_with_large_vector() {
        // Test with large vector for performance/stability check
        let data: Vec<_> = (0..10000).collect();
        let vector = ColumnVector::new(&data);
        assert_eq!(vector.height(), 10000);
    }

    #[test]
    fn test_height_after_operations() {
        // Test height remains consistent after operations
        let data1 = vec![1, 2, 3];
        let data2 = vec![4, 5, 6];
        let vec1 = ColumnVector::new(&data1);
        let vec2 = ColumnVector::new(&data2);
        let result = vec1 + vec2;
        assert_eq!(result.height(), 3);
    }

    #[test]
    fn test_height_matches_vector_length() {
        // Test height matches underlying vector length
        let data = vec![1.1, 2.2, 3.3];
        let vector = ColumnVector::new(&data);
        assert_eq!(vector.height(), data.len());
    }

    #[test]
    fn test_height_zeros_constructor() {
        // Test height returns correct value when using zeros constructor
        let height = 5;
        let vector = ColumnVector::<i32>::zeros(height);
        assert_eq!(vector.height(), height);
    }

    #[test]
    fn test_height_after_scalar_multiplication() {
        // Test height remains unchanged after scalar multiplication
        let data = vec![1, 2, 3];
        let vector = ColumnVector::new(&data);
        let result = vector * 2;
        assert_eq!(result.height(), 3);
    }

    #[test]
    fn test_height_after_vector_subtraction() {
        // Test height remains unchanged after vector subtraction
        let data1 = vec![1, 2, 3];
        let data2 = vec![4, 5, 6];
        let vec1 = ColumnVector::new(&data1);
        let vec2 = ColumnVector::new(&data2);
        let result = vec1 - vec2;
        assert_eq!(result.height(), 3);
    }

    #[test]
    fn test_height_after_to_matrix_conversion() {
        // Test height matches before and after matrix conversion
        let data = vec![1, 2, 3];
        let vector = ColumnVector::new(&data);
        let matrix = vector.to_matrix();
        assert_eq!(vector.height(), matrix.height());
    }
}

