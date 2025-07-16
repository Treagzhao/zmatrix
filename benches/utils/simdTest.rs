#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_simd_vector_add_empty_input() {
        // 测试空输入情况
        let input: Vec<f64> = vec![];
        let result = simd_vector_add(&input, 5.0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_simd_vector_add_single_element() {
        // 测试单元素向量
        let input = vec![2.0];
        let result = simd_vector_add(&input, 3.0);
        assert_relative_eq!(result[0], 5.0);
    }

    #[test]
    fn test_simd_vector_add_exact_simd_chunk() {
        // 测试恰好是SIMD块大小的输入(4个元素)
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let result = simd_vector_add(&input, 10.0);
        let expected = vec![11.0, 12.0, 13.0, 14.0];
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_relative_eq!(r, e);
        }
    }

    #[test]
    fn test_simd_vector_add_with_remainder() {
        // 测试带剩余元素的输入(非4倍数长度)
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = simd_vector_add(&input, 10.0);
        let expected = vec![11.0, 12.0, 13.0, 14.0, 15.0];
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_relative_eq!(r, e);
        }
    }

    #[test]
    fn test_simd_vector_add_large_input() {
        // 测试大输入(超过100个元素)
        let size = 101;
        let input: Vec<f64> = (0..size).map(|x| x as f64).collect();
        let value = 10.5;
        let result = simd_vector_add(&input, value);

        for (i, x) in result.iter().enumerate() {
            assert_relative_eq!(*x, i as f64 + value);
        }
    }

    #[test]
    fn test_simd_vector_add_zero_value() {
        // 测试加0的特殊情况
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = simd_vector_add(&input, 0.0);
        let expected = input.clone();
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_relative_eq!(r, e);
        }
    }

    #[test]
    fn test_simd_vector_add_negative_value() {
        // 测试负值加法
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let result = simd_vector_add(&input, -2.0);
        let expected = vec![-1.0, 0.0, 1.0, 2.0];
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_relative_eq!(r, e);
        }
    }

    #[test]
    fn test_simd_vector_add_extreme_values() {
        // 测试极端值输入
        let input = vec![f64::MAX, f64::MIN, f64::EPSILON];
        let value = 1.0;
        let result = simd_vector_add(&input, value);

        assert_relative_eq!(result[0], f64::MAX + 1.0);
        assert_relative_eq!(result[1], f64::MIN + 1.0);
        assert_relative_eq!(result[2], f64::EPSILON + 1.0);
    }
}


