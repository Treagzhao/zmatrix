//! 按位表决支持（TMR / MNV）。
//!
//! 三机冗余（Triple Modular Redundancy）或多数表决（Majority Vote）的核心动作是
//! **逐位比较各机的状态**。本模块提供在 `zmatrix` 类型上安全做这件事的基础设施。
//!
//! # 为什么不能直接 `memcmp`
//!
//! Rust 的默认布局（`repr(Rust)`）不保证字段顺序，也不保证没有填充字节。
//! 本库的标量物理量都是
//!
//! ```text
//! struct Distance { default_type: DistanceType, v: f64 }
//! ```
//!
//! 的形状，实测布局是 `[f64 8 字节][tag 1 字节][padding 7 字节]`，`size_of` = 16。
//! 那 7 个 padding 字节**内容是栈/堆残留**，不是零：
//!
//! ```text
//! Distance::from_m(5.0)      字节: [.. .. .. .. .. .. 14 40 | 00 01 bb 9c 01 00 00 00]
//! Distance::from_m(2.5+2.5)  字节: [.. .. .. .. .. .. 14 40 | 00 3d b7 6b 01 00 00 00]
//!                                                    ↑ f64 相同        ↑ padding 不同
//! ```
//!
//! 两者 `==` 判定相等，`memcmp` 判定不等。若用 `memcmp` 表决，三台冗余机
//! 只要栈布局或 ASLR 不同就会**永久互相否决**——而且看起来像是"三台都坏了"，
//! 极难定位。
//!
//! 因此本模块的 [`TmrValue::tmr_bits`] 逐字段取值、逐字段转位型，
//! **padding 完全不参与**。
//!
//! # 为什么用位型而不是 `==`
//!
//! 两个方向都会出错：
//!
//! | 情形 | `a == b` | 位型比较 | 表决应取 |
//! |---|---|---|---|
//! | `NaN` vs `NaN`（同位型） | `false` | `true` | **位型**（否则两个合法 NaN 被判分歧） |
//! | `-0.0` vs `+0.0` | `true` | `false` | **位型**（否则掩盖真实的状态差异） |
//!
//! 第一条是硬性的：用 `==` 表决会让任何含 NaN 的状态永久无法达成一致。
//! 第二条是关键取舍：三机跑同样的代码和输入，正常不会出现 ±0 分歧；
//! 真出现了，说明某台机器确实算出了不同的东西，报出来比忽略掉好。
//!
//! # 三机必须使用相同的构造单位
//!
//! 表决比较的是**内部存储值**（[`PhysicalQuantity::raw_value`]），不做单位归一化。
//! `Mass::from_g(500.0)` 与 `Mass::from_kg(0.5)` 语义相等但内部表示不同，
//! 表决会判为分歧。这是期望行为——内部表示不同就是不同步。
//!
//! [`PhysicalQuantity`]: crate::physics::basic::PhysicalQuantity
//! [`PhysicalQuantity::raw_value`]: crate::physics::basic::PhysicalQuantity::raw_value

use crate::physics::basic::{PhysicalQuantity, Vector3, VectorQuantity};
use crate::spatial_geometry::cos_matrix::CosMatrix;
use crate::spatial_geometry::euler_angles::EulerAngles;
use crate::spatial_geometry::quaternion::Quaternion;

/// 单次表决可携带的最大 64 位字数。
///
/// 当前最大的参与者是 [`CosMatrix`]（9 个 `f64`）。留到 9 是因为它就是上限，
/// 若要支持更大的类型，改这个常量即可——容量不够时 [`TmrBits::push`] 会 panic
/// 而不是静默截断。
pub const MAX_TMR_WORDS: usize = 9;

/// 参与表决的位型序列。
///
/// 定长、`Copy`、无堆分配——表决在故障处理路径上执行，不能依赖分配器。
#[derive(Clone, Copy, Debug)]
pub struct TmrBits {
    words: [u64; MAX_TMR_WORDS],
    len: u8,
}

impl TmrBits {
    /// 空序列，容量为 [`MAX_TMR_WORDS`]。
    pub const fn new() -> Self {
        Self {
            words: [0; MAX_TMR_WORDS],
            len: 0,
        }
    }

    /// 追加一个 64 位字。
    ///
    /// # Panics
    ///
    /// 超过 [`MAX_TMR_WORDS`] 时 panic。**故意不静默截断**：截断会让两个不同的
    /// 状态产生相同的表决键，那是最坏的一种错误——表决会"通过"。
    pub fn push(&mut self, word: u64) {
        assert!(
            (self.len as usize) < MAX_TMR_WORDS,
            "TmrBits 容量 {} 不足，参与表决的类型过大",
            MAX_TMR_WORDS
        );
        self.words[self.len as usize] = word;
        self.len += 1;
    }

    /// 追加一个 `f64` 的位型（保留 NaN payload 与 ±0 符号）。
    pub fn push_f64(&mut self, value: f64) {
        self.push(value.to_bits());
    }

    /// 有效字数。
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 有效部分的只读切片，**不含**未使用的尾部槽位。
    pub fn as_slice(&self) -> &[u64] {
        &self.words[..self.len as usize]
    }
}

impl Default for TmrBits {
    fn default() -> Self {
        Self::new()
    }
}

// 手写而不是 derive：derive 的 PartialEq 会比较整个 `words` 数组，包括未使用的
// 尾部槽位。虽然 `push` 保证了它们不被写入（初始为 0），但只比较有效部分是更
// 直接的语义——两个长度为 3 的键，其第 4..9 槽位的内容与相等性无关。
impl PartialEq for TmrBits {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for TmrBits {}

impl std::hash::Hash for TmrBits {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// 可安全参与按位表决的类型。
///
/// 实现者承诺：`tmr_bits` 返回的序列**只由逻辑状态决定**，与内存布局、padding、
/// 分配地址无关。两个逻辑状态相同的实例必须给出相同的序列。
pub trait TmrValue {
    /// 逐字段取出位型，padding 不参与。
    ///
    /// 相同逻辑状态必须给出相同结果（对同一位型）；不同逻辑状态必须给出不同结果。
    fn tmr_bits(&self) -> TmrBits;
}

// 所有标量物理量：单个 f64 的位型。
//
// 这里必须用 `raw_value()` 而不是 `default_unit_value()`——后者带单位换算，
// 会把 g 与 kg 两种表示归一化，从而掩盖表示层的分歧。
impl<T: PhysicalQuantity> TmrValue for T {
    fn tmr_bits(&self) -> TmrBits {
        let mut bits = TmrBits::new();
        bits.push_f64(self.raw_value());
        bits
    }
}

// 三维向量：三个分量依次取出。
//
// 注意这里不能用 `to_array()`——那个方法同样走 `default_unit_value()`，带换算。
// 约束是 `VectorQuantity + Default`（前者是 `PhysicalQuantity` 的子 trait，
// 后者是 `Vector3` 自身的结构约束）。
impl<T: VectorQuantity + Default> TmrValue for Vector3<T> {
    fn tmr_bits(&self) -> TmrBits {
        let mut bits = TmrBits::new();
        bits.push_f64(self.x.raw_value());
        bits.push_f64(self.y.raw_value());
        bits.push_f64(self.z.raw_value());
        bits
    }
}

// 四元数：q0..q3 四个裸 f64，无 padding（size_of = 32 = 4×8），但统一走逐字段
// 路径，避免将来加字段时布局变化引起静默错误。
impl TmrValue for Quaternion {
    fn tmr_bits(&self) -> TmrBits {
        let mut bits = TmrBits::new();
        bits.push_f64(self.q0);
        bits.push_f64(self.q1);
        bits.push_f64(self.q2);
        bits.push_f64(self.q3);
        bits
    }
}

// 方向余弦矩阵：9 个 f64。内部是 `[f64; 9]`，无 padding，仍然逐元素取。
//
// `to_array()` 返回 `[[f64; 3]; 3]`（按行分组），不是扁平数组。
// 这里按行优先展平，与内部存储顺序一致。
impl TmrValue for CosMatrix {
    fn tmr_bits(&self) -> TmrBits {
        let mut bits = TmrBits::new();
        let rows = self.to_array();
        for row in rows.iter() {
            for v in row.iter() {
                bits.push_f64(*v);
            }
        }
        bits
    }
}

// 4×3 矩阵（如 `ksi_matrix` 的输出）。4 行 × 3 列 = 12 个字，超过当前容量，
// 因此不实现——需要时提高 MAX_TMR_WORDS。

// 欧拉角：三个分量依次取出。
//
// 需求原文只列了「EulerAngles 补 Copy/Clone」，没列它的 TmrValue。但补 Copy 的
// 目的正是让它能参与表决，只给 Copy 而不给表决键等于做了一半，所以一并实现。
impl TmrValue for EulerAngles {
    fn tmr_bits(&self) -> TmrBits {
        let mut bits = TmrBits::new();
        bits.push_f64(self.yaw.raw_value());
        bits.push_f64(self.pitch.raw_value());
        bits.push_f64(self.roll.raw_value());
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::{Angular, Distance, Mass, Velocity};

    /// 三取二表决，返回多数键；全不一致返回 `None`。
    ///
    /// 这是使用方最可能照抄的形状，放在这里既便于测试，也充当示例。
    fn vote3(a: TmrBits, b: TmrBits, c: TmrBits) -> Option<TmrBits> {
        if a == b {
            Some(a)
        } else if a == c {
            Some(a)
        } else if b == c {
            Some(b)
        } else {
            None
        }
    }

    // ===== 基础类型 =====

    #[test]
    fn test_tmr_bits_new_is_empty() {
        let bits = TmrBits::new();
        assert!(bits.is_empty());
        assert_eq!(bits.len(), 0);
        assert_eq!(bits.as_slice(), &[] as &[u64]);
    }

    #[test]
    fn test_tmr_bits_push_and_slice() {
        let mut bits = TmrBits::new();
        bits.push(0xDEAD_BEEF);
        bits.push_f64(1.0);
        assert_eq!(bits.len(), 2);
        assert_eq!(bits.as_slice()[0], 0xDEAD_BEEF);
        assert_eq!(bits.as_slice()[1], 1.0f64.to_bits());
    }

    #[test]
    #[should_panic(expected = "容量")]
    fn test_tmr_bits_overflow_panics() {
        let mut bits = TmrBits::new();
        for i in 0..=MAX_TMR_WORDS {
            bits.push(i as u64);
        }
    }

    // 相等性只看有效部分：长度不同则不等，长度相同则逐字比较
    #[test]
    fn test_tmr_bits_equality_uses_only_valid_words() {
        let mut a = TmrBits::new();
        a.push(1);
        a.push(2);

        let mut b = TmrBits::new();
        b.push(1);
        b.push(2);

        let mut c = TmrBits::new();
        c.push(1);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ===== PhysicalQuantity blanket =====

    #[test]
    fn test_scalar_bits_equal_for_same_value() {
        let a = Distance::from_m(5.0);
        let b = Distance::from_m(5.0);
        assert_eq!(a.tmr_bits(), b.tmr_bits());
    }

    #[test]
    fn test_scalar_bits_differ_for_different_value() {
        let a = Distance::from_m(5.0);
        let b = Distance::from_m(5.000_000_000_000_001);
        assert_ne!(a.tmr_bits(), b.tmr_bits());
    }

    // ★ 核心：不同构造路径的同一语义值，位型必须相同
    //
    // 这一条正是 memcmp 会失败的场景。
    #[test]
    fn test_scalar_bits_ignore_padding_across_construction_paths() {
        let a = Distance::from_m(5.0);
        let b = Distance::from_m(2.5 + 2.5);
        let c = Distance::default() + Distance::from_m(5.0);

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a.tmr_bits(), b.tmr_bits(), "同语义值位型必须相同");
        assert_eq!(a.tmr_bits(), c.tmr_bits());
    }

    // ★ raw_value 与 default_unit_value 的差别会体现在表决上
    #[test]
    fn test_units_are_not_normalized() {
        let in_grams = Mass::from_g(500.0);
        let in_kg = Mass::from_kg(0.5);

        // 数值上相等（换算后）
        assert_eq!(
            in_grams.default_unit_value(),
            in_kg.default_unit_value(),
            "default_unit_value 做归一化，两者相同"
        );

        // 但内部表示不同，表决必须判为分歧
        assert_ne!(
            in_grams.raw_value(),
            in_kg.raw_value(),
            "raw_value 保留原始单位"
        );
        assert_ne!(
            in_grams.tmr_bits(),
            in_kg.tmr_bits(),
            "不同单位表示同一物理量，表决必须判为分歧"
        );
    }

    // ===== NaN 与 ±0 =====

    // ★ NaN：`==` 判不等，位型判等。用 `==` 表决会永久无法达成一致。
    #[test]
    fn test_nan_bits_equal_though_value_not_equal() {
        let a = Distance::from_m(f64::NAN);
        let b = Distance::from_m(f64::NAN);

        assert_ne!(a, b, "NaN != NaN，数值比较判不等");
        assert_eq!(a.tmr_bits(), b.tmr_bits(), "同位型 NaN 的表决键必须相同");
    }

    // 不同 payload 的 NaN 位型不同，表决应判为分歧
    #[test]
    fn test_nan_with_different_payload_differs() {
        // f64::NAN 的位型是 0x7ff8000000000000
        // 构造一个 payload 不同的 quiet NaN
        let other_nan = f64::from_bits(0x7ff8_0000_0000_0001);

        let a = Distance::from_m(f64::NAN);
        let b = Distance::from_m(other_nan);

        assert_ne!(
            a.tmr_bits(),
            b.tmr_bits(),
            "不同 payload 的 NaN 位型不同，应判为分歧"
        );
    }

    // ★ ±0：`==` 判等，位型判不等
    #[test]
    fn test_signed_zero_bits_differ_though_value_equal() {
        let pos = Distance::from_m(0.0);
        let neg = Distance::from_m(-0.0);

        assert_eq!(pos, neg, "0.0 == -0.0，数值比较判等");
        assert_ne!(
            pos.tmr_bits(),
            neg.tmr_bits(),
            "+0.0 与 -0.0 位型不同，表决判为分歧（这是有意为之）"
        );
        assert_eq!(pos.raw_value().to_bits(), 0x0000_0000_0000_0000);
        assert_eq!(neg.raw_value().to_bits(), 0x8000_0000_0000_0000);
    }

    // ===== 复合类型 =====

    #[test]
    fn test_vector3_bits() {
        let a = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let b = a;
        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_eq!(a.tmr_bits().len(), 3);

        let c = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(4.0),
        );
        assert_ne!(a.tmr_bits(), c.tmr_bits());
    }

    // 分量顺序敏感：交换 y 与 z 必须产生不同的键
    #[test]
    fn test_vector3_bits_are_order_sensitive() {
        let a = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let b = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(3.0),
            Distance::from_m(2.0),
        );
        assert_ne!(a.tmr_bits(), b.tmr_bits());
    }

    #[test]
    fn test_quaternion_bits() {
        let a = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let b = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_eq!(a.tmr_bits().len(), 4);

        let c = Quaternion::new(1.0, 0.0, 0.0, 1e-16);
        assert_ne!(a.tmr_bits(), c.tmr_bits());
    }

    // q 与 -q 表示同一姿态，但位型不同——表决会判分歧
    //
    // 这是数学事实，不是缺陷：内部状态确实不同。姿态层面的等价需要调用方
    // 在表决前自行归一化（如统一到 q0 >= 0 的半球），本库不做这个隐式转换。
    #[test]
    fn test_quaternion_q_and_negative_q_differ() {
        let a = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let neg = Quaternion::new(-1.0, 0.0, 0.0, 0.0);
        assert_ne!(a.tmr_bits(), neg.tmr_bits());
    }

    #[test]
    fn test_cos_matrix_bits() {
        let a = CosMatrix::default();
        let b = CosMatrix::default();
        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_eq!(a.tmr_bits().len(), 9);
        assert_eq!(a.tmr_bits().len(), MAX_TMR_WORDS, "恰好用满容量");
    }

    #[test]
    fn test_cos_matrix_bits_differ_on_single_element() {
        let a = CosMatrix::default();
        // 用 f64::EPSILON 而不是手写 1e-16：后者小于 1.0 附近的机器精度，
        // 1.0 + 1e-16 会被舍入回 1.0，两个矩阵实际完全相同，测试就失效了。
        let b = CosMatrix::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0 + f64::EPSILON],
        ]);
        // CosMatrix 没有 PartialEq，用 to_array 验证构造确实不同
        assert_ne!(
            a.to_array(),
            b.to_array(),
            "构造出的矩阵必须真的不同，否则本测试无意义"
        );
        assert_ne!(a.tmr_bits(), b.tmr_bits());
    }

    // ===== 表决本身 =====

    // 三取二：两台一致、一台偏差 → 选出多数
    #[test]
    fn test_majority_vote_three_units() {
        let good1 = Distance::from_m(5.0);
        let good2 = Distance::from_m(5.0);
        let faulty = Distance::from_m(5.1);

        let result = vote3(good1.tmr_bits(), good2.tmr_bits(), faulty.tmr_bits());
        assert_eq!(result, Some(good1.tmr_bits()), "应选出健康值");
    }

    // 表决函数的四类结果：三种多数情形 + 全不一致
    #[test]
    fn test_vote_all_branches() {
        let x = Distance::from_m(1.0).tmr_bits();
        let y = Distance::from_m(2.0).tmr_bits();
        let z = Distance::from_m(3.0).tmr_bits();

        // a == b（第三台坏）
        assert_eq!(vote3(x.clone(), x.clone(), y.clone()), Some(x.clone()));
        // a == c（第二台坏）
        assert_eq!(vote3(x.clone(), y.clone(), x.clone()), Some(x.clone()));
        // b == c（第一台坏）
        assert_eq!(vote3(y.clone(), x.clone(), x.clone()), Some(x.clone()));
        // 三台全不同
        assert_eq!(vote3(x, y, z), None);
    }

    // 三机全部一致（含 NaN 状态）必须通过
    #[test]
    fn test_majority_passes_when_all_nan() {
        let a = Angular::from_rad(f64::NAN);
        let b = Angular::from_rad(f64::NAN);
        let c = Angular::from_rad(f64::NAN);
        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_eq!(b.tmr_bits(), c.tmr_bits());
    }

    // TmrBits 可用作 HashMap 的键（表决结果缓存、分歧统计等场景）
    #[test]
    fn test_tmr_bits_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Distance::from_m(1.0).tmr_bits());
        set.insert(Distance::from_m(1.0).tmr_bits()); // 重复
        assert_eq!(set.len(), 1, "相同键应去重");

        set.insert(Distance::from_m(2.0).tmr_bits());
        assert_eq!(set.len(), 2);

        // 与 Eq 一致：等值必须同 hash
        assert!(set.contains(&Distance::from_m(1.0).tmr_bits()));
        assert!(!set.contains(&Distance::from_m(3.0).tmr_bits()));
    }

    #[test]
    fn test_tmr_bits_default_is_empty() {
        let d = TmrBits::default();
        assert!(d.is_empty());
        assert_eq!(d, TmrBits::new());
    }

    // 含 padding 的类型参与表决时，三台用不同构造路径也算一致
    #[test]
    fn test_vote_tolerates_padding_differences() {
        let unit1 = Velocity::from_m_per_sec(7.5);
        let unit2 = Velocity::from_m_per_sec(15.0 / 2.0);
        let unit3 = Velocity::from_m_per_sec(5.0 + 2.5);

        assert_eq!(unit1.tmr_bits(), unit2.tmr_bits());
        assert_eq!(unit2.tmr_bits(), unit3.tmr_bits());
    }

    // ============================================================
    // 需求第 5 条要求的四类覆盖
    // ============================================================

    // ===== ① 无 padding 类型：可直接按字节比较 =====
    //
    // `Coef` 只有 `v: f64` 一个字段，`size_of` = 8，没有 padding 空间。
    // 这类类型上 memcmp 与位型比较等价，是唯一可以放心用字节比较的。
    #[test]
    fn test_no_padding_type_coef() {
        use crate::physics::basic::Coef;
        use std::mem::size_of;

        assert_eq!(
            size_of::<Coef>(),
            size_of::<f64>(),
            "Coef 应与裸 f64 同宽，没有 padding 空间"
        );

        let a = Coef::new(1.5);
        let b = Coef::new(1.5);
        let c = Coef::new(1.5 + f64::EPSILON);

        // 无 padding ⇒ 字节相等与位型相等等价
        let bytes = |v: &Coef| unsafe {
            std::slice::from_raw_parts(v as *const Coef as *const u8, size_of::<Coef>()).to_vec()
        };
        assert_eq!(bytes(&a), bytes(&b));
        assert_ne!(bytes(&a), bytes(&c));

        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_ne!(a.tmr_bits(), c.tmr_bits());
    }

    // ===== ② 含 padding 的 newtype：禁止 memcmp =====
    //
    // 这是本模块存在的主要理由。
    //
    // 先说清楚一个容易误解的点：**按值赋值会连同 padding 一起复制**。
    // `let b = a;` 是整块内存拷贝，padding 跟着走。所以 padding 只有在
    // **构造的那一瞬间**才可能是未初始化的残留值。
    //
    // 实测（macOS / aarch64，debug）：
    //
    //   Distance::from_m(5.0)      字节: [.. .. .. .. .. .. 14 40 | 00 01 bb 9c 01 00 00 00]
    //   Distance::from_m(2.5+2.5)  字节: [.. .. .. .. .. .. 14 40 | 00 3d b7 6b 01 00 00 00]
    //                                                      ↑ f64 相同      ↑ padding 不同
    //
    // 但 padding 的具体内容取决于栈布局，**无法在测试里稳定复现**——
    // 尝试用 `MaybeUninit` 写入脏值时，`write` 会把整个值（含源值的 padding）
    // 拷进去，脏值反被覆盖。所以我不断言"字节必然不同"，而是断言下面三条
    // 结构性事实；它们合起来已经足以说明 memcmp 不可用。
    #[test]
    fn test_padded_newtype_cannot_be_memcmped() {
        use std::mem::size_of;

        // ① 类型确实有 padding 空间：16 字节里只有 9 字节是有意义的
        //    （f64 8 字节 + enum tag 1 字节），另外 7 字节不受控
        assert_eq!(size_of::<Distance>(), 16, "标量物理量占 16 字节");
        assert_eq!(size_of::<f64>(), 8);
        assert!(
            size_of::<Distance>() > size_of::<f64>(),
            "多出来的字节就是 tag + padding，字节比较会把它们算进去"
        );

        // ② tmr_bits 只取 1 个字 = 只取 raw_value 的位型，
        //    16 字节里的 tag 与 padding **完全不参与**
        assert_eq!(
            Distance::from_m(5.0).tmr_bits().len(),
            1,
            "只应取 f64 一个字段，padding 不参与"
        );

        // ③ 因此两个语义相同的值必然给出相同键，与它们的内存字节无关
        let paths = [
            Distance::from_m(5.0),
            Distance::from_m(2.5 + 2.5),
            Distance::default() + Distance::from_m(5.0),
            Distance::from_m(10.0) / 2.0,
        ];
        let first = paths[0].tmr_bits();
        for (i, p) in paths.iter().enumerate() {
            assert_eq!(p, &paths[0], "第 {} 个构造路径的语义值应相同", i);
            assert_eq!(p.tmr_bits(), first, "第 {} 个构造路径的键应相同", i);
        }
    }

    // ===== ③ NaN 位型 =====
    //
    // 已在 test_nan_bits_equal_though_value_not_equal / test_nan_with_different_payload_differs
    // 覆盖，这里补一条：NaN 的符号位也参与位型。
    #[test]
    fn test_nan_sign_bit_participates() {
        let pos_nan = f64::from_bits(0x7ff8_0000_0000_0000); // 正 NaN
        let neg_nan = f64::from_bits(0xfff8_0000_0000_0000); // 负 NaN

        assert_ne!(
            pos_nan.to_bits(),
            neg_nan.to_bits(),
            "符号位不同则位型不同"
        );

        let a = Distance::from_m(pos_nan);
        let b = Distance::from_m(neg_nan);
        assert_ne!(a.tmr_bits(), b.tmr_bits());
    }

    // 全 NaN 状态的三机表决必须通过（`==` 会让它永远失败）
    #[test]
    fn test_all_nan_units_agree_under_vote() {
        let units = [
            Distance::from_m(f64::NAN),
            Distance::from_m(f64::NAN),
            Distance::from_m(f64::NAN),
        ];
        let bits: Vec<_> = units.iter().map(|u| u.tmr_bits()).collect();
        assert_eq!(bits[0], bits[1]);
        assert_eq!(bits[1], bits[2]);

        // 对照：用 == 判等会失败
        assert_ne!(units[0], units[1]);
    }

    // ===== ④ -0.0 vs +0.0 =====
    //
    // 已在 test_signed_zero_bits_differ_though_value_equal 覆盖。
    // 这里补一条：这与 IEEE-754 的语义一致（两者数值相等但表示不同），
    // 表决取位型是有意为之，不是 bug。
    #[test]
    fn test_signed_zero_across_complex_types() {
        // 向量层面同样成立
        let v_pos = Vector3::new(
            Distance::from_m(0.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );
        let v_neg = Vector3::new(
            Distance::from_m(-0.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );
        assert_ne!(v_pos.tmr_bits(), v_neg.tmr_bits());

        // 四元数层面
        let q_pos = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let q_neg = Quaternion::new(1.0, -0.0, 0.0, 0.0);
        assert_ne!(
            q_pos.tmr_bits(),
            q_neg.tmr_bits(),
            "同一姿态的 ±0 表示在位型上不同"
        );
    }

    // ===== EulerAngles =====

    #[test]
    fn test_euler_angles_bits() {
        let a = EulerAngles::new(
            Angular::from_rad(0.1),
            Angular::from_rad(0.2),
            Angular::from_rad(0.3),
        );
        // Copy 语义：复制后仍可用（若未实现 Copy，这行编译不过）
        let b = a;
        let _ = a;

        assert_eq!(a.tmr_bits(), b.tmr_bits());
        assert_eq!(a.tmr_bits().len(), 3);

        let c = EulerAngles::new(
            Angular::from_rad(0.1),
            Angular::from_rad(0.2),
            Angular::from_rad(0.4),
        );
        assert_ne!(a.tmr_bits(), c.tmr_bits());
    }

    #[test]
    fn test_euler_angles_derive() {
        let a = EulerAngles::new(
            Angular::from_rad(0.1),
            Angular::from_rad(0.2),
            Angular::from_rad(0.3),
        );
        // Debug
        let s = format!("{:?}", a);
        assert!(s.contains("EulerAngles"));
        // Clone / Copy
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
