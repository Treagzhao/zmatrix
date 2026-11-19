//! 判零阈值。
//!
//! 该阈值的**存储方式随 `tmr` feature 变化**：
//!
//! | feature | 存储 | 运行时可否修改 | 定制方式 | `get_flt64_zero` |
//! |---|---|---|---|---|
//! | 关（默认） | `static RwLock<f64>` | 是，`set_flt64_zero` | 运行时调用 setter | 普通 `fn` |
//! | 开 | `const f64` | 否 | `ZMATRIX_FLT64_ZERO` 环境变量，构建时生效 | `const fn` |
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
//! # 需要别的阈值怎么办
//!
//! 定制发生在**构建时**，不是运行时：
//!
//! ```bash
//! ZMATRIX_FLT64_ZERO=1e-12 cargo build --features tmr
//! ```
//!
//! 值由 `build.rs` 解析校验后写成字面量。**三机必须用同一个值构建**——如果一台
//! 二进制是 `1e-12`、另一台是默认的 `1e-14`，判零结论就会不同，表决会持续报分歧。
//! 这也是为什么阈值不做成运行时参数：运行时能改的东西，三台就有机会改得不一致。
//!
//! 非法值（非数字、NaN、无穷、非正数）会让**构建直接失败**，而不是退回默认值——
//! 阈值配错的表现是判零结论悄悄改变，这类错误极难从仿真结果反推。
//!
//! 默认（不启用 `tmr`）构建保持原有的可变语义，不受影响。

#[cfg(feature = "tmr")]
mod imp {
    // 由 build.rs 生成，内容为 `pub const FLT64_ZERO: f64 = <字面量>;`
    // 连同它的文档注释（会注明当前值来自默认还是环境变量）。
    include!(concat!(env!("OUT_DIR"), "/flt64_zero.rs"));

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
    /// assert_eq!(TOL, FLT64_ZERO);
    /// ```
    ///
    /// 具体数值取决于构建时的 `ZMATRIX_FLT64_ZERO` 环境变量（未设置则 `1e-14`），
    /// 所以上面的例子只断言两者一致，不断言具体值。
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
    /// 启用 `tmr` feature 后此函数不存在（阈值变为编译期常量，改用
    /// `ZMATRIX_FLT64_ZERO` 环境变量在构建时指定）。
    pub fn set_flt64_zero(value: f64) {
        *FLT64_ZERO.write().unwrap() = value;
    }
}

pub use imp::*;

#[cfg(all(test, feature = "tmr"))]
mod tests_tmr {
    use super::*;

    // 不硬编码具体数值：它可能是默认的 1e-14，也可能是构建时通过
    // `ZMATRIX_FLT64_ZERO` 定制的。断言的是「机制正确」而不是「值等于某个常数」。
    #[test]
    fn test_get_matches_constant() {
        assert_eq!(get_flt64_zero(), FLT64_ZERO);
    }

    // 编译期可用：若它不是 const fn，这行无法通过编译
    #[test]
    fn test_usable_in_const_context() {
        const TOL: f64 = get_flt64_zero();
        assert_eq!(TOL, FLT64_ZERO);

        const _: f64 = FLT64_ZERO;
    }

    // 值必须为正：判零逻辑写的是 `norm > get_flt64_zero()`，
    // 若为 0，全零向量会走「非零」分支导致除零；若为负，条件恒真。
    #[test]
    fn test_tolerance_is_positive() {
        assert!(FLT64_ZERO > 0.0);
        assert!(FLT64_ZERO.is_finite());
    }

    // 未定制时应等于默认值。用编译期常量判断，因为构建时的环境变量
    // 无法在运行时读到——这个常量由 build.rs 一并生成。
    //
    // 写成短路求值而不是 `if`：`if` 的收尾大括号会被算作一条未执行的
    // 分支路径（条件为假时跳转的目标），白白拉低覆盖率。
    // 定制时该值仍须为正且有限，那一条由 `test_tolerance_is_positive` 无条件覆盖。
    #[test]
    fn test_default_value_when_not_customized() {
        assert!(!IS_DEFAULT || FLT64_ZERO == 1e-14);
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
