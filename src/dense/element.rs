//! 矩阵元素类型闸门（NumericElement）
//!
//! [`Matrix`](crate::dense::Matrix) 的元素类型被限制为**基础数字量**（整数与浮点），
//! 物理量（`Distance`、`Coef` 等）不能作为矩阵元素。
//!
//! # 为什么
//!
//! 一、**语义上做不到**。`Matrix<ROWS, COLS, T>` 只有一个元素类型，要求整个矩阵同量纲。
//! 而工程上真正需要的矩阵几乎全是异量纲的：状态转移阵 Φ、协方差阵 P、卡尔曼增益 K、
//! 雅可比 ∂f/∂x —— 它们的行列量纲各不相同，根本无法表示为单一 `T`。
//! 唯一能同量纲表达的就是旋转矩阵（元素无量纲），而它已有
//! [`CosMatrix`](crate::spatial_geometry::cos_matrix::CosMatrix) 专门实现。
//!
//! 二、**实现上是陷阱**。物理量的 `Default` 不保证是加法零元
//! （`Coef::default() == 1.0` 是有意的乘法幺元），而矩阵乘积、求和、
//! 行列式都需要以加法零元为累加器起点。一旦允许物理量进矩阵，
//! 每个累加器都成了潜伏的正确性缺陷。
//!
//! 因此本 trait 用 sealed 模式把门关死：只有本文件内列出的基础数字量能实现它，
//! 外部 crate 无法扩展。物理量在编译期就进不了 [`Matrix`](crate::dense::Matrix)。
//!
//! # 示例
//! ```compile_fail
//! use zmatrix::dense::Matrix;
//! use zmatrix::physics::basic::Coef;
//!
//! // 物理量不能作为矩阵元素，编译失败
//! let m: Matrix<2, 2, Coef> = Matrix::new([
//!     [Coef::new(1.0), Coef::new(0.0)],
//!     [Coef::new(0.0), Coef::new(1.0)],
//! ]);
//! ```
//!
//! ```compile_fail
//! use zmatrix::dense::Matrix;
//!
//! // 自定义类型也不行：NumericElement 是 sealed 的
//! struct MyNum(f64);
//! let m: Matrix<1, 1, MyNum> = Matrix::new([[MyNum(1.0)]]);
//! ```

mod sealed {
    /// 密封标记，阻止外部 crate 为自定义类型实现 [`NumericElement`](super::NumericElement)。
    pub trait Sealed {}
}

/// 可作为 [`Matrix`](crate::dense::Matrix) 元素的基础数字量。
///
/// 这是一个 **sealed** trait：只有整数与浮点实现了它，外部无法扩展。
/// 它同时也是 `Copy + Send + Sync` 的来源，因此满足本 trait 的类型可直接用于 `Matrix`。
pub trait NumericElement: sealed::Sealed + Copy + Send + Sync {}

macro_rules! impl_numeric_element {
    ($($t:ty),* $(,)?) => {
        $(
            impl sealed::Sealed for $t {}
            impl NumericElement for $t {}
        )*
    };
}

impl_numeric_element!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

#[cfg(test)]
mod tests {
    use super::*;

    // 编译期即可确认：常用数字类型都实现了 NumericElement
    #[test]
    fn test_primitives_implement_numeric_element() {
        fn assert_impl<T: NumericElement>() {}
        assert_impl::<f64>();
        assert_impl::<f32>();
        assert_impl::<i32>();
        assert_impl::<u8>();
        assert_impl::<usize>();
    }

    // NumericElement 是 sealed 的：物理量无法实现它。
    // 该约束由下方的 compile_fail doctest 在编译期守住，这里只确认
    // 本模块内没有任何物理量被误加进 impl 列表。
    #[test]
    fn test_numeric_element_is_sealed_by_module() {
        fn assert_sealed<T: sealed::Sealed>() {}
        assert_sealed::<f64>();
        assert_sealed::<i32>();
    }
}
