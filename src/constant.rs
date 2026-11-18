//! 判零阈值。
//!
//! 该阈值的**存储方式随 `tmr` feature 变化**：
//!
//! | feature | 存储 | 可修改 | `get_flt64_zero` |
//! |---|---|---|---|
//! | 关（默认） | `static RwLock<f64>` | 是，`set_flt64_zero` | 普通 `fn` |
//! | 开 | `const f64` | 否 | `const fn` |
//!
//! # 为什么 `tmr` 下要改成常量
//!
//! 三机冗余（TMR / MNV）场景下，运行时可变的全局容差构成两个问题：
//!
//! 1. **热路径锁** —— [`get_flt64_zero`] 被 `normalize()` 等判零逻辑调用，
//!    每次都要获取读锁。
//!
//! 2. **共模故障源**（更严重）—— 三台冗余机若读到不同的阈值，判零结论就不同，
//!    表决会永久分歧；更糟的是这个变量可被任何代码（包括已被 SEU 破坏的代码）
//!    在运行时改写，而且**三台会同时被改写**，因为跑的是同一份逻辑。
//!    这正是共模故障的定义：冗余无法覆盖的失效模式。
//!
//! 改成编译期常量后，三台机器的行为必然一致——阈值固化在二进制里，
//! 运行时没有任何路径能改变它。
//!
//! 默认（不启用 `tmr`）构建保持原有的可变语义，不受影响。

#[cfg(feature = "tmr")]
mod imp {
    /// 判定「浮点零」的阈值。
    ///
    /// 绝对值小于此值的量在判零/归一化时被视为零。默认 `1e-14`。
    ///
    /// 编译期常量：启用 `tmr` 时不能也不应在运行时修改，见模块文档。
    pub const FLT64_ZERO: f64 = 1e-14;

    /// 返回判零阈值，等价于 [`FLT64_ZERO`]。
    ///
    /// 启用 `tmr` 时是 `const fn`：编译期求值，运行时零开销，
    /// 且可用于常量上下文。
    ///
    /// ```
    /// use zmatrix::constant::{get_flt64_zero, FLT64_ZERO};
    ///
    /// assert_eq!(get_flt64_zero(), FLT64_ZERO);
    ///
    /// // 可用于常量上下文
    /// const TOL: f64 = get_flt64_zero();
    /// assert_eq!(TOL, 1e-14);
    /// ```
    pub const fn get_flt64_zero() -> f64 {
        FLT64_ZERO
    }
}

#[cfg(not(feature = "tmr"))]
mod imp {
    use std::sync::RwLock;

    static FLT64_ZERO: RwLock<f64> = RwLock::new(1e-14);

    /// 返回判零阈值。默认 `1e-14`。
    pub fn get_flt64_zero() -> f64 {
        *FLT64_ZERO.read().unwrap()
    }

    /// 修改判零阈值。
    ///
    /// 全局生效，影响所有判零/归一化路径。多线程下需自行保证调用时机。
    ///
    /// 启用 `tmr` feature 后此函数不存在（阈值变为编译期常量）。
    pub fn set_flt64_zero(value: f64) {
        *FLT64_ZERO.write().unwrap() = value;
    }
}

pub use imp::*;

#[cfg(all(test, feature = "tmr"))]
mod tests_tmr {
    use super::*;

    #[test]
    fn test_flt64_zero_is_constant() {
        assert_eq!(get_flt64_zero(), 1e-14);
        assert_eq!(get_flt64_zero(), FLT64_ZERO);
    }

    // 编译期可用：若它不是 const fn，这行无法通过编译
    #[test]
    fn test_usable_in_const_context() {
        const TOL: f64 = get_flt64_zero();
        assert_eq!(TOL, 1e-14);

        const _: f64 = FLT64_ZERO;
    }

    // 值必须为正：判零逻辑写的是 `norm > get_flt64_zero()`，
    // 若为 0，全零向量会走「非零」分支导致除零；若为负，条件恒真。
    #[test]
    fn test_tolerance_is_positive() {
        assert!(FLT64_ZERO > 0.0);
        assert!(FLT64_ZERO.is_finite());
    }
}

#[cfg(all(test, not(feature = "tmr")))]
mod tests_mutable {
    use super::*;

    #[test]
    fn test_get_set_flt64_zero() {
        assert_eq!(get_flt64_zero(), 1e-14);

        set_flt64_zero(1e-10);
        assert_eq!(get_flt64_zero(), 1e-10);

        set_flt64_zero(0.0);
        assert_eq!(get_flt64_zero(), 0.0);

        set_flt64_zero(1.0);
        assert_eq!(get_flt64_zero(), 1.0);

        // 恢复默认值，避免影响其他测试
        set_flt64_zero(1e-14);
        assert_eq!(get_flt64_zero(), 1e-14);
    }
}
