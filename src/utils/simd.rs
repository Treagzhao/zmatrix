#![feature(portable_simd)]

use std::arch::is_aarch64_feature_detected;
// 必须放在文件顶部
use std::simd::{f64x4, Simd};

pub fn simd_vector_add<T>(src: &[T], value: T) -> Vec<T>
where
    T: SimdArithmetic + Copy + From<f32> + From<i32>,
    Simd<T, 8>: SimdArithmetic,
{
    let mut result = src.to_vec(); // 克隆数据
    let value_simd = f64x4::splat(value);

    // SIMD 处理块
    let mut chunks = result.chunks_exact_mut(4);

    for chunk in &mut chunks {
        let mut simd_arr = f64x4::from_slice(chunk);
        simd_arr += value_simd;
        simd_arr.copy_to_slice(chunk);
    }

    let remainder = chunks.into_remainder();



    // 处理剩余元素
    for x in remainder {
        *x += value;
    }

    result
}

fn simd_width() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") { 512 }
        else if is_x86_feature_detected!("avx2") { 256 }
        else if is_x86_feature_detected!("sse2") { 128 }
        else { 0 }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") { 128 }
        else { 0 }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_simd_width(){
        let width = simd_width();
        println!("{:?}",width);
    }

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





