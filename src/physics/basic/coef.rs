use std::ops::{Add, Div, Mul, Neg, Sub};
use std::time::Duration;
use approx::{AbsDiffEq, RelativeEq};
use super::*;
impl Coef {
    pub fn new(v: f64) -> Self {
        Coef { v }
    }
    pub fn get_value(&self) -> f64 {
        self.v
    }
}

impl Add for Coef {
    type Output = Coef;
    fn add(self, rhs: Self) -> Self::Output {
        Coef { v: self.v + rhs.v }
    }
}

// 引用-引用 与 混合引用：Coef 加法
impl<'a, 'b> Add<&'b Coef> for &'a Coef {
    type Output = Coef;
    fn add(self, rhs: &'b Coef) -> Self::Output { Coef { v: self.v + rhs.v } }
}
impl<'a> Add<&'a Coef> for Coef {
    type Output = Coef;
    fn add(self, rhs: &'a Coef) -> Self::Output { Coef { v: self.v + rhs.v } }
}
impl<'a> Add<Coef> for &'a Coef {
    type Output = Coef;
    fn add(self, rhs: Coef) -> Self::Output { Coef { v: self.v + rhs.v } }
}

impl Sub for Coef {
    type Output = Coef;
    fn sub(self, rhs: Self) -> Self::Output {
        Coef { v: self.v - rhs.v }
    }
}

// 引用-引用 与 混合引用：Coef 减法
impl<'a, 'b> Sub<&'b Coef> for &'a Coef {
    type Output = Coef;
    fn sub(self, rhs: &'b Coef) -> Self::Output { Coef { v: self.v - rhs.v } }
}
impl<'a> Sub<&'a Coef> for Coef {
    type Output = Coef;
    fn sub(self, rhs: &'a Coef) -> Self::Output { Coef { v: self.v - rhs.v } }
}
impl<'a> Sub<Coef> for &'a Coef {
    type Output = Coef;
    fn sub(self, rhs: Coef) -> Self::Output { Coef { v: self.v - rhs.v } }
}

impl Mul for Coef {
    type Output = Coef;
    fn mul(self, rhs: Self) -> Self::Output {
        Coef { v: self.v * rhs.v }
    }
}

// 引用-引用 与 混合引用：Coef 乘法
impl<'a, 'b> Mul<&'b Coef> for &'a Coef {
    type Output = Coef;
    fn mul(self, rhs: &'b Coef) -> Self::Output { Coef { v: self.v * rhs.v } }
}
impl<'a> Mul<&'a Coef> for Coef {
    type Output = Coef;
    fn mul(self, rhs: &'a Coef) -> Self::Output { Coef { v: self.v * rhs.v } }
}
impl<'a> Mul<Coef> for &'a Coef {
    type Output = Coef;
    fn mul(self, rhs: Coef) -> Self::Output { Coef { v: self.v * rhs.v } }
}
impl Mul<Coef> for f64 {
    type Output = Coef;

    fn mul(self, rhs: Coef) -> Self::Output {
        rhs * self
    }
}

impl PhysicalQuantity for Coef {
    fn as_any(&self) -> &dyn Any {
        self
}
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.v
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "Coef".to_string()
    }
}


impl Sub<Coef> for f64 {
    type Output = Coef;
    fn sub(self, rhs: Coef) -> Self::Output {
        Coef { v: self - rhs.v }
    }
}

impl Mul<Duration> for Coef {
    type Output = Coef;

    fn mul(self, rhs: Duration) -> Self::Output {
        let v = self.get_value() * rhs.as_secs_f64();
        Coef { v }
    }
}

impl Mul<chrono::TimeDelta> for Coef {
    type Output = Coef;

    fn mul(self, rhs: chrono::TimeDelta) -> Self::Output {
        let v = self.get_value() * time_delta_to_secs_f64(&rhs);
        Coef { v }
    }
}

impl Div for Coef {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        Coef { v: self.v / rhs.v }
    }
}

// 引用-引用 与 混合引用：Coef 除法
impl<'a, 'b> Div<&'b Coef> for &'a Coef {
    type Output = Coef;
    fn div(self, rhs: &'b Coef) -> Self::Output { Coef { v: self.v / rhs.v } }
}
impl<'a> Div<&'a Coef> for Coef {
    type Output = Coef;
    fn div(self, rhs: &'a Coef) -> Self::Output { Coef { v: self.v / rhs.v } }
}
impl<'a> Div<Coef> for &'a Coef {
    type Output = Coef;
    fn div(self, rhs: Coef) -> Self::Output { Coef { v: self.v / rhs.v } }
}

impl Div<f64> for Coef {
    type Output = Coef;
    fn div(self, rhs: f64) -> Self::Output {
        Coef { v: self.v / rhs }
    }
}

impl Div<Coef> for f64 {
    type Output = Coef;

    fn div(self, rhs: Coef) -> Self::Output {
        Coef { v: self / rhs.v }
    }
}

// Coef 是无量纲量，加裸 f64 不存在单位歧义，直接作用于数值。
// 详见 `physics::basic` 模块文档「物理量与裸 f64 的运算」。
impl Add<f64> for Coef {
    type Output = Coef;
    fn add(self, rhs: f64) -> Self::Output {
        Coef { v: self.v + rhs }
    }
}

impl Sub<f64> for Coef {
    type Output = Coef;
    fn sub(self, rhs: f64) -> Self::Output {
        Coef { v: self.v - rhs }
    }
}

impl Mul<f64> for Coef {
    type Output = Coef;
    fn mul(self, rhs: f64) -> Self::Output {
        Coef { v: self.v * rhs }
    }
}


impl Neg for Coef {
    type Output = Coef;

    fn neg(self) -> Self::Output {
        Coef::new(self.get_value() * -1.0)
    }
}


impl From<Angular> for Coef {
    fn from(value: Angular) -> Self {
        Coef::new(value.as_rad())
    }
}

impl AbsDiffEq for Coef {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.v.abs_diff_eq(&other.v, epsilon)
    }
}

impl RelativeEq for Coef {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.v.relative_eq(&other.v, epsilon, max_relative)
    }
}

impl AbsDiffEq<f64> for Coef {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &f64, epsilon: Self::Epsilon) -> bool {
        self.v.abs_diff_eq(other, epsilon)
    }
}

impl RelativeEq<f64> for Coef {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(&self, other: &f64, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.v.relative_eq(other, epsilon, max_relative)
    }
}

impl PartialEq<f64> for Coef {
    fn eq(&self, other: &f64) -> bool {
        self.v == *other
    }
}

impl PartialOrd<f64> for Coef {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(other)
    }
}

// ── Coef × 物理量（反向乘法）────────────────────────────────────────────────
//
// 契约：`Coef` 与任何一种物理量的乘法必须**双向对称**。
// 各物理量文件中定义了 `impl Mul<Coef> for X`（X × Coef），
// 这里补齐 `impl Mul<X> for Coef`（Coef × X），后者统一委托给前者，
// 因此两条方向的单位换算语义完全一致 —— 结果一律以该物理量的**基准单位**返回
// （例如 `Coef::new(2.0) * Distance::from_km(1.0)` 得到 2000 m，单位信息不再保留为 km）。
//
// ⚠️ 新增物理量时，若它在自己的文件中实现了 `Mul<Coef>`，必须同时在下方清单里登记，
// 否则 `Coef * X` 会编译失败，破坏对称性。
macro_rules! impl_coef_mul_quantity {
    ($($type:ty),* $(,)?) => {
        $(
            impl Mul<$type> for Coef {
                type Output = $type;

                #[inline]
                fn mul(self, rhs: $type) -> Self::Output {
                    // 委托给各物理量的 X × Coef 实现，保证单位语义一致
                    rhs * self
                }
            }
        )*
    };
}

impl_coef_mul_quantity!(
    Distance,
    Velocity,
    Acceleration,
    Angular,
    AngularVelocity,
    AngularAcceleration,
    Area,
    Volume,
    Mass,
    Momentum,
    AngularMomentum,
    Force,
    Torque,
    Energy,
    Power,
    MagneticInduction,
    MagneticMoment,
    MagneticAngularVelocity,
    ElectricCurrent,
    ElectricPotential,
    ElectricResistance,
    ElectricCharge,
    ElectricCapacitance,
    ElectricConductance,
);





#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_new() {
        let coef = Coef::new(1.0);
        assert_eq!(coef.v, 1.0);
    }

    #[test]
    fn test_get_value() {
        let coef = Coef::new(1.0);
        assert_eq!(coef.get_value(), 1.0);

        let c = -coef;
        assert_eq!(c.v, -1.0);
    }

    #[test]
    fn test_calculate_coef() {
        let coef = Coef::new(1.0);
        let result = coef + Coef::new(2.0);
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef + 2.0;
        assert_eq!(result.get_value(), 4.0);

        let coef = Coef::new(2.0);
        let result = coef - Coef::new(3.0);
        assert_eq!(result.get_value(), -1.0);

        let coef = Coef::new(2.0);
        let result = coef - 1.0;
        assert_eq!(result.get_value(), 1.0);

        let coef = Coef::new(2.0);
        let result = coef * Coef::new(3.0);
        assert_eq!(result.get_value(), 6.0);

        let coef = Coef::new(2.0);
        let result = coef * 1.5;
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef / Coef::new(0.5);
        assert_eq!(result.get_value(), 4.0);

        let coef = Coef::new(2.0);
        let result = coef / Coef::new(0.5);

        assert_eq!(result.get_value(), 4.0);
    }

    #[test]
    fn test_add() {
        let coef = Coef::new(1.0);
        let result = coef + Coef::new(2.0);
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef + 2.0;
        assert_eq!(result.get_value(), 4.0);
    }
    #[test]
    fn test_sub() {
        let coef = Coef::new(1.0);
        let result = coef - Coef::new(2.0);
        assert_eq!(result.get_value(), -1.0);

        let coef = Coef::new(2.0);
        let result = coef - 6.0;
        assert_eq!(result.get_value(), -4.0);

        let coef = Coef::new(1.0);
        let result = 2.0 - coef;
        assert_eq!(result.get_value(), 1.0);
    }

    #[test]
    fn test_mul() {
        let coef = Coef::new(1.0);
        let result = coef * Coef::new(2.0);
        assert_eq!(result.get_value(), 2.0);

        let coef = Coef::new(2.0);
        let result = coef * 3.0;
        assert_eq!(result.get_value(), 6.0);

        let coef = Coef::new(2.0);
        let result = coef * Duration::from_secs(10);
        assert_eq!(result.get_value(), 20.0);

        let coef = Coef::new(2.0);
        let td = time_delta_from_secs_f64(10.0);
        let result = coef * td;
        assert_eq!(result.get_value(), 20.0);

        // 负时间
        let neg_td = time_delta_from_secs_f64(-3.0);
        let result_neg = coef * neg_td;
        assert_eq!(result_neg.get_value(), -6.0);

        let coef = Coef::new(2.0);
        let result = 2.0 * coef;
        assert_eq!(result.get_value(), 4.0);
    }

    #[test]
    fn test_div() {
        let coef = Coef::new(1.0);
        let result = coef / Coef::new(2.0);
        assert_eq!(result.get_value(), 0.5);

        let coef = Coef::new(2.0);
        let result = coef / 4.0;
        assert_eq!(result.get_value(), 0.5);

        let a: f64 = 10.0;
        let coef = Coef::new(2.0);
        let result = a / coef;
        assert_eq!(result.get_value(), 5.0);

    }

    #[test]
    fn test_is_zero() {
        let coef = Coef::new(0.0);
        assert_eq!(coef.is_zero(), true);
        let coef = Coef::new(1.0);
        assert_eq!(coef.is_zero(), false);
    }

    #[test]
    fn test_as_any() {
        let coef = Coef::new(1.0);
        let result = coef.as_any();
        let _c = result.downcast_ref::<Coef>().unwrap();
    }

    #[test]
    fn test_default_unit_value() {
        let coef = Coef::new(1.0);
        assert_eq!(coef.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut coef = Coef::new(1.0);
        coef.set_value(2.0);
        assert_eq!(coef.get_value(), 2.0);
    }

    #[test]
    fn test_from() {
        let angular = Angular::from_rad(1.0);
        let coef = Coef::from(angular);
        assert_eq!(coef.get_value(), 1.0);
    }

    #[test]
    fn test_partial_eq_f64() {
        let coef = Coef::new(1.0);
        
        // 测试 Coef == f64
        assert_eq!(coef, 1.0);
        assert_ne!(coef, 2.0);
        
        // 测试边界情况
        let zero_coef = Coef::new(0.0);
        assert_eq!(zero_coef, 0.0);
        assert_ne!(zero_coef, 0.1);
        
        // 测试负数
        let negative_coef = Coef::new(-5.5);
        assert_eq!(negative_coef, -5.5);
        assert_ne!(negative_coef, 5.5);
        
        // 测试小数
        let decimal_coef = Coef::new(3.14159);
        assert_eq!(decimal_coef, 3.14159);
        assert_ne!(decimal_coef, 3.14);
        
        // 测试很大的数
        let large_coef = Coef::new(1e10);
        assert_eq!(large_coef, 1e10);
        assert_ne!(large_coef, 1e9);
        
        // 测试很小的数
        let small_coef = Coef::new(1e-10);
        assert_eq!(small_coef, 1e-10);
        assert_ne!(small_coef, 1e-9);
    }

    #[test]
    fn test_partial_ord_f64() {
        let coef = Coef::new(5.0);
        
        // 测试比较操作
        assert!(coef > 4.0);
        assert!(coef >= 5.0);
        assert!(coef >= 4.0);
        assert!(coef < 6.0);
        assert!(coef <= 5.0);
        assert!(coef <= 6.0);
        
        // 测试负数比较
        let negative_coef = Coef::new(-3.0);
        assert!(negative_coef < 0.0);
        assert!(negative_coef < -2.0);
        assert!(negative_coef > -4.0);
        
        // 测试零值比较
        let zero_coef = Coef::new(0.0);
        assert!(zero_coef == 0.0);
        assert!(zero_coef > -1.0);
        assert!(zero_coef < 1.0);
    }

    #[test]
    fn test_abs_diff_eq() {
        let coef1 = Coef::new(1.0);
        let coef2 = Coef::new(1.0 + f64::EPSILON);
        let coef3 = Coef::new(1.0 + 2.0 * f64::EPSILON);
        
        // 测试相等情况
        assert!(coef1.abs_diff_eq(&coef1, f64::EPSILON));
        
        // 测试在误差范围内的差异
        assert!(coef1.abs_diff_eq(&coef2, f64::EPSILON));
        
        // 测试超出误差范围的差异
        assert!(!coef1.abs_diff_eq(&coef3, f64::EPSILON));
        
        // 测试零值
        let zero_coef = Coef::new(0.0);
        assert!(zero_coef.abs_diff_eq(&zero_coef, f64::EPSILON));
        
        // 测试负数
        let negative_coef = Coef::new(-1.0);
        assert!(negative_coef.abs_diff_eq(&negative_coef, f64::EPSILON));
    }

    #[test]
    fn test_relative_eq() {
        let coef1 = Coef::new(1.0);
        let coef2 = Coef::new(1.0 + f64::EPSILON);
        let coef3 = Coef::new(1.0 + 2.0 * f64::EPSILON);
        
        // 测试相等情况
        assert!(coef1.relative_eq(&coef1, f64::EPSILON, f64::EPSILON));
        
        // 测试在相对误差范围内的差异
        assert!(coef1.relative_eq(&coef2, f64::EPSILON, f64::EPSILON));
        
        // 测试超出相对误差范围的差异
        assert!(!coef1.relative_eq(&coef3, f64::EPSILON, f64::EPSILON));
        
        // 测试大数值的相对比较
        let large_coef1 = Coef::new(1e10);
        let large_coef2 = Coef::new(1e10 + 1e6);
        assert!(large_coef1.relative_eq(&large_coef2, f64::EPSILON, 1e-4));
    }

    #[test]
    fn test_abs_diff_eq_f64() {
        let coef = Coef::new(1.0);
        let f64_val1 = 1.0 + f64::EPSILON;
        let f64_val2 = 1.0 + 2.0 * f64::EPSILON;
        
        // 测试相等情况
        assert!(coef.abs_diff_eq(&1.0, f64::EPSILON));
        
        // 测试在误差范围内的差异
        assert!(coef.abs_diff_eq(&f64_val1, f64::EPSILON));
        
        // 测试超出误差范围的差异
        assert!(!coef.abs_diff_eq(&f64_val2, f64::EPSILON));
        
        // 测试零值
        let zero_coef = Coef::new(0.0);
        assert!(zero_coef.abs_diff_eq(&0.0, f64::EPSILON));
        
        // 测试负数
        let negative_coef = Coef::new(-1.0);
        assert!(negative_coef.abs_diff_eq(&-1.0, f64::EPSILON));
    }

    #[test]
    fn test_relative_eq_f64() {
        let coef = Coef::new(1.0);
        let f64_val1 = 1.0 + f64::EPSILON;
        let f64_val2 = 1.0 + 2.0 * f64::EPSILON;
        
        // 测试相等情况
        assert!(coef.relative_eq(&1.0, f64::EPSILON, f64::EPSILON));
        
        // 测试在相对误差范围内的差异
        assert!(coef.relative_eq(&f64_val1, f64::EPSILON, f64::EPSILON));
        
        // 测试超出相对误差范围的差异
        assert!(!coef.relative_eq(&f64_val2, f64::EPSILON, f64::EPSILON));
        
        // 测试大数值的相对比较
        let large_coef = Coef::new(1e10);
        assert!(large_coef.relative_eq(&(1e10 + 1e6), f64::EPSILON, 1e-4));
    }

    #[test]
    fn test_default_epsilon() {
        // 测试 AbsDiffEq 的默认 epsilon
        assert_eq!(<Coef as AbsDiffEq>::default_epsilon(), f64::EPSILON);
        
        // 测试 RelativeEq 的默认 max_relative
        assert_eq!(<Coef as RelativeEq>::default_max_relative(), f64::EPSILON);
        
        // 测试 AbsDiffEq<f64> 的默认 epsilon
        assert_eq!(<Coef as AbsDiffEq<f64>>::default_epsilon(), f64::EPSILON);
        
        // 测试 RelativeEq<f64> 的默认 max_relative
        assert_eq!(<Coef as RelativeEq<f64>>::default_max_relative(), f64::EPSILON);
    }

    #[test]
    fn test_edge_cases() {
        // 测试无穷大
        let inf_coef = Coef::new(f64::INFINITY);
        assert!(inf_coef.get_value().is_infinite());
        assert!(!inf_coef.get_value().is_finite());
        
        // 测试负无穷大
        let neg_inf_coef = Coef::new(f64::NEG_INFINITY);
        assert!(neg_inf_coef.get_value().is_infinite());
        assert!(!neg_inf_coef.get_value().is_finite());
        
        // 测试 NaN
        let nan_coef = Coef::new(f64::NAN);
        assert!(nan_coef.get_value().is_nan());
        assert!(!nan_coef.get_value().is_finite());
        
        // 测试最大和最小值
        let max_coef = Coef::new(f64::MAX);
        let min_coef = Coef::new(f64::MIN);
        assert!(max_coef.get_value().is_finite());
        assert!(min_coef.get_value().is_finite());
    }

    #[test]
    fn test_division_by_zero() {
        // 测试除以零的情况
        let coef = Coef::new(1.0);
        let zero_coef = Coef::new(0.0);
        
        // 这应该产生无穷大
        let result = coef / zero_coef;
        assert!(result.get_value().is_infinite());
        
        // 测试 f64 除以零系数
        let result = 1.0 / zero_coef;
        assert!(result.get_value().is_infinite());
    }

    #[test]
    fn test_overflow_underflow() {
        // 测试乘法溢出
        let large_coef = Coef::new(f64::MAX);
        let result = large_coef * 2.0;
        assert!(result.get_value().is_infinite());
        
        // 测试加法溢出
        let max_coef = Coef::new(f64::MAX);
        let result = max_coef + f64::MAX;
        assert!(result.get_value().is_infinite());
        
        // 测试减法下溢
        let min_coef = Coef::new(f64::MIN);
        let result = min_coef - f64::MAX;
        assert!(result.get_value().is_infinite());
    }

    #[test]
    fn test_coef_ref_ops() {
        let a = Coef::new(2.0);
        let b = Coef::new(3.0);
        let s = &a + &b;
        assert_relative_eq!(s.get_value(), 5.0);

        let d = &a - &b;
        assert_relative_eq!(d.get_value(), -1.0);

        let m = &a * &b;
        assert_relative_eq!(m.get_value(), 6.0);

        let q = &a / &Coef::new(0.5);
        assert_relative_eq!(q.get_value(), 4.0);

        // 混合引用：四则
        let s2 = a + &b;
        assert_relative_eq!(s2.get_value(), 5.0);
        let s3 = &a + b;
        assert_relative_eq!(s3.get_value(), 5.0);

        let b2 = Coef::new(3.0);
        let d2 = a - &b2;
        assert_relative_eq!(d2.get_value(), -1.0);
        let d3 = &a - b2;
        assert_relative_eq!(d3.get_value(), -1.0);

        let m2 = a * &Coef::new(3.0);
        assert_relative_eq!(m2.get_value(), 6.0);
        let m3 = &a * Coef::new(3.0);
        assert_relative_eq!(m3.get_value(), 6.0);

        let q2 = a / &Coef::new(0.5);
        assert_relative_eq!(q2.get_value(), 4.0);
        let q3 = &a / Coef::new(0.5);
        assert_relative_eq!(q3.get_value(), 4.0);
    }

    // ── Coef × 物理量 的对称性 ──────────────────────────────────────────────
    //
    // 回归：曾经只实现了 `X × Coef`，`Coef × X` 全部缺失，
    // 导致 `Coef::new(2.0) * Distance::from_m(3.0)` 编译失败。
    // 下面逐个类型验证两条方向都存在且数值一致。
    macro_rules! assert_symmetry {
        ($($type:ty => $make:expr, $read:expr, $tol:expr;)*) => {
            $(
                {
                    let q: $type = $make;
                    let c = Coef::new(2.0);

                    // 正向：物理量 × Coef
                    let forward: $type = q * c;
                    // 反向：Coef × 物理量
                    let reverse: $type = c * q;

                    assert_relative_eq!($read(forward), $read(reverse), epsilon = $tol);
                    // 与期望值的对照（2 倍）
                    assert_relative_eq!($read(forward), 2.0 * $read(q), epsilon = $tol);
                }
            )*
        };
    }

    #[test]
    fn test_coef_mul_quantity_symmetry() {
        assert_symmetry! {
            Distance                => Distance::from_m(3.0),                    |x: Distance| x.as_m(), 1e-12;
            Velocity                => Velocity::from_m_per_sec(3.0),            |x: Velocity| x.as_m_per_sec(), 1e-12;
            Acceleration            => Acceleration::from_m_per_s2(3.0),         |x: Acceleration| x.as_m_per_s2(), 1e-12;
            Angular                 => Angular::from_rad(3.0),                   |x: Angular| x.as_rad(), 1e-12;
            AngularVelocity         => AngularVelocity::from_rad_per_second(3.0),|x: AngularVelocity| x.as_rad_per_second(), 1e-12;
            AngularAcceleration     => AngularAcceleration::from_rad_per_second2(3.0), |x: AngularAcceleration| x.as_rad_per_second2(), 1e-12;
            Area                    => Area::from_m2(3.0),                       |x: Area| x.as_m2(), 1e-12;
            Volume                  => Volume::from_m3(3.0),                     |x: Volume| x.as_m3(), 1e-12;
            Mass                    => Mass::from_kg(3.0),                       |x: Mass| x.as_kg(), 1e-12;
            Momentum                => Momentum::from_kg_m_s(3.0),               |x: Momentum| x.as_kg_m_s(), 1e-12;
            AngularMomentum         => AngularMomentum::from_nms(3.0),           |x: AngularMomentum| x.as_nms(), 1e-12;
            Force                   => Force::from_newton(3.0),                  |x: Force| x.as_newton(), 1e-12;
            Torque                  => Torque::from_nm(3.0),                     |x: Torque| x.as_nm(), 1e-12;
            Energy                  => Energy::from_joule(3.0),                  |x: Energy| x.as_joule(), 1e-12;
            Power                   => Power::from_watt(3.0),                    |x: Power| x.as_watt(), 1e-12;
            MagneticInduction       => MagneticInduction::from_tesla(3.0),       |x: MagneticInduction| x.as_tesla(), 1e-12;
            MagneticMoment          => MagneticMoment::from_am2(3.0),           |x: MagneticMoment| x.as_am2(), 1e-12;
            MagneticAngularVelocity => MagneticAngularVelocity::from_tesla_rad_per_second(3.0), |x: MagneticAngularVelocity| x.as_tesla_rad_per_second(), 1e-12;
            ElectricCurrent         => ElectricCurrent::from_a(3.0),             |x: ElectricCurrent| x.as_a(), 1e-12;
            ElectricPotential       => ElectricPotential::from_v(3.0),           |x: ElectricPotential| x.as_v(), 1e-12;
            ElectricResistance      => ElectricResistance::from_ohm(3.0),        |x: ElectricResistance| x.as_ohm(), 1e-12;
            ElectricCharge          => ElectricCharge::from_coulomb(3.0),        |x: ElectricCharge| x.as_coulomb(), 1e-12;
            ElectricCapacitance     => ElectricCapacitance::from_farad(3.0),     |x: ElectricCapacitance| x.as_farad(), 1e-12;
            ElectricConductance     => ElectricConductance::from_siemens(3.0),   |x: ElectricConductance| x.as_siemens(), 1e-12;
        }
    }

    // 反向乘法的单位语义：与正向一致，一律以基准单位返回。
    // from_km(1.0) 与 from_m(1000.0) 是两个不同存储单位的等值量，
    // 乘同一个 Coef 后必须得到相同结果 —— 这条曾因换算缺失而被破坏。
    #[test]
    fn test_coef_mul_quantity_unit_independent() {
        let c = Coef::new(2.0);

        let from_m = Distance::from_m(1000.0);
        let from_km = Distance::from_km(1.0);
        assert_relative_eq!(from_m.as_m(), from_km.as_m(), epsilon = 1e-12);

        // 正向
        assert_relative_eq!((from_m * c).as_m(), 2000.0, epsilon = 1e-12);
        assert_relative_eq!((from_km * c).as_m(), 2000.0, epsilon = 1e-12);

        // 反向（本次新增的路径）
        assert_relative_eq!((c * from_m).as_m(), 2000.0, epsilon = 1e-12);
        assert_relative_eq!((c * from_km).as_m(), 2000.0, epsilon = 1e-12);
    }
}