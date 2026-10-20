use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    AngularAcceleration, AngularMomentum, AngularVelocity, Coef, Inertia, InertiaType,
    PhysicalQuantity, Torque,
};

/// kg·mm² 与 kg·m² 的换算系数：1 mm² = 1e-6 m²
const KG_MM2_TO_KG_M2: f64 = 1.0e-6;

impl Default for Inertia {
    fn default() -> Self {
        Self::from_kg_m2(0.0)
    }
}

impl PhysicalQuantity for Inertia {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_m2()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "Inertia (kg·m²)".to_string()
    }
}

impl Inertia {
    pub fn from_kg_m2(v: f64) -> Self {
        Self {
            default_type: InertiaType::KgM2,
            v,
        }
    }

    /// 从 kg·mm² 构造。
    ///
    /// 工程图纸与 CAD 报告的转动惯量常以 kg·mm² 给出（aods 的 `mass_sim_para`
    /// 中 `J_kgmm2` 即此单位），比 kg·m² 大 10⁶ 倍。
    pub fn from_kg_mm2(v: f64) -> Self {
        Self {
            default_type: InertiaType::KgMm2,
            v,
        }
    }

    pub fn as_kg_m2(&self) -> f64 {
        match self.default_type {
            InertiaType::KgM2 => self.v,
            InertiaType::KgMm2 => self.v * KG_MM2_TO_KG_M2,
        }
    }

    pub fn as_kg_mm2(&self) -> f64 {
        match self.default_type {
            InertiaType::KgM2 => self.v / KG_MM2_TO_KG_M2,
            InertiaType::KgMm2 => self.v,
        }
    }
}

impl Add for Inertia {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() + rhs.as_kg_m2())
    }
}

// 引用-引用 与 混合引用：Inertia 加法
impl<'a, 'b> Add<&'b Inertia> for &'a Inertia {
    type Output = Inertia;
    fn add(self, rhs: &'b Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() + rhs.as_kg_m2()) }
}
impl<'a> Add<&'a Inertia> for Inertia {
    type Output = Inertia;
    fn add(self, rhs: &'a Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() + rhs.as_kg_m2()) }
}
impl<'a> Add<Inertia> for &'a Inertia {
    type Output = Inertia;
    fn add(self, rhs: Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() + rhs.as_kg_m2()) }
}

impl Sub for Inertia {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() - rhs.as_kg_m2())
    }
}

impl<'a, 'b> Sub<&'b Inertia> for &'a Inertia {
    type Output = Inertia;
    fn sub(self, rhs: &'b Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() - rhs.as_kg_m2()) }
}
impl<'a> Sub<&'a Inertia> for Inertia {
    type Output = Inertia;
    fn sub(self, rhs: &'a Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() - rhs.as_kg_m2()) }
}
impl<'a> Sub<Inertia> for &'a Inertia {
    type Output = Inertia;
    fn sub(self, rhs: Inertia) -> Self::Output { Inertia::from_kg_m2(self.as_kg_m2() - rhs.as_kg_m2()) }
}

impl Neg for Inertia {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_kg_m2(-self.as_kg_m2())
    }
}

impl Mul<f64> for Inertia {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() * rhs)
    }
}

impl Div<f64> for Inertia {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() / rhs)
    }
}

impl Mul<Inertia> for f64 {
    type Output = Inertia;
    fn mul(self, rhs: Inertia) -> Self::Output {
        Inertia::from_kg_m2(self * rhs.as_kg_m2())
    }
}

impl Div<Inertia> for f64 {
    type Output = Coef;
    fn div(self, rhs: Inertia) -> Self::Output {
        Coef::new(self / rhs.as_kg_m2())
    }
}

impl Mul<Coef> for Inertia {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() * rhs.get_value())
    }
}

impl Div<Coef> for Inertia {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_kg_m2(self.as_kg_m2() / rhs.get_value())
    }
}

// 转动惯量 × 角速度 = 角动量：J·ω = L
impl Mul<AngularVelocity> for Inertia {
    type Output = AngularMomentum;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        AngularMomentum::from_kg_m2_per_second(self.as_kg_m2() * rhs.as_rad_per_second())
    }
}

impl<'a> Mul<&'a AngularVelocity> for Inertia {
    type Output = AngularMomentum;
    fn mul(self, rhs: &'a AngularVelocity) -> Self::Output {
        AngularMomentum::from_kg_m2_per_second(self.as_kg_m2() * rhs.as_rad_per_second())
    }
}

impl<'a> Mul<AngularVelocity> for &'a Inertia {
    type Output = AngularMomentum;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        AngularMomentum::from_kg_m2_per_second(self.as_kg_m2() * rhs.as_rad_per_second())
    }
}

// 转动惯量 × 角加速度 = 力矩：J·ω̇ = M
impl Mul<AngularAcceleration> for Inertia {
    type Output = Torque;
    fn mul(self, rhs: AngularAcceleration) -> Self::Output {
        Torque::from_nm(self.as_kg_m2() * rhs.as_rad_per_second2())
    }
}

impl<'a> Mul<&'a AngularAcceleration> for Inertia {
    type Output = Torque;
    fn mul(self, rhs: &'a AngularAcceleration) -> Self::Output {
        Torque::from_nm(self.as_kg_m2() * rhs.as_rad_per_second2())
    }
}

impl<'a> Mul<AngularAcceleration> for &'a Inertia {
    type Output = Torque;
    fn mul(self, rhs: AngularAcceleration) -> Self::Output {
        Torque::from_nm(self.as_kg_m2() * rhs.as_rad_per_second2())
    }
}

// 力矩 ÷ 转动惯量 = 角加速度：M / J = ω̇
impl Div<Inertia> for Torque {
    type Output = AngularAcceleration;
    fn div(self, rhs: Inertia) -> Self::Output {
        AngularAcceleration::from_rad_per_second2(self.as_nm() / rhs.as_kg_m2())
    }
}

impl<'a> Div<&'a Inertia> for Torque {
    type Output = AngularAcceleration;
    fn div(self, rhs: &'a Inertia) -> Self::Output {
        AngularAcceleration::from_rad_per_second2(self.as_nm() / rhs.as_kg_m2())
    }
}

impl<'a> Div<Inertia> for &'a Torque {
    type Output = AngularAcceleration;
    fn div(self, rhs: Inertia) -> Self::Output {
        AngularAcceleration::from_rad_per_second2(self.as_nm() / rhs.as_kg_m2())
    }
}

// 力矩 ÷ 角加速度 = 转动惯量：M / ω̇ = J
//
// 与 `AngularMomentum / AngularVelocity = Inertia` 是同一个物理量的两条独立
// 计算路径（静态的 L/ω 与动态的 M/ω̇），两者必须给出同一量纲和同一数值。
impl Div<AngularAcceleration> for Torque {
    type Output = Inertia;
    fn div(self, rhs: AngularAcceleration) -> Self::Output {
        Inertia::from_kg_m2(self.as_nm() / rhs.as_rad_per_second2())
    }
}

impl<'a> Div<&'a AngularAcceleration> for Torque {
    type Output = Inertia;
    fn div(self, rhs: &'a AngularAcceleration) -> Self::Output {
        Inertia::from_kg_m2(self.as_nm() / rhs.as_rad_per_second2())
    }
}

impl<'a> Div<AngularAcceleration> for &'a Torque {
    type Output = Inertia;
    fn div(self, rhs: AngularAcceleration) -> Self::Output {
        Inertia::from_kg_m2(self.as_nm() / rhs.as_rad_per_second2())
    }
}

impl<'a, 'b> Div<&'b AngularAcceleration> for &'a Torque {
    type Output = Inertia;
    fn div(self, rhs: &'b AngularAcceleration) -> Self::Output {
        Inertia::from_kg_m2(self.as_nm() / rhs.as_rad_per_second2())
    }
}

// 角动量 ÷ 转动惯量 = 角速度：L / J = ω
//
// 与 `J·ω = L` 互为逆运算。飞轮/反作用轮仿真里这是主力路径：
// 已知轮子当前角动量与惯量，反解转速。
impl Div<Inertia> for AngularMomentum {
    type Output = AngularVelocity;
    fn div(self, rhs: Inertia) -> Self::Output {
        AngularVelocity::from_rad_per_second(self.as_nms() / rhs.as_kg_m2())
    }
}

impl<'a> Div<&'a Inertia> for AngularMomentum {
    type Output = AngularVelocity;
    fn div(self, rhs: &'a Inertia) -> Self::Output {
        AngularVelocity::from_rad_per_second(self.as_nms() / rhs.as_kg_m2())
    }
}

impl<'a> Div<Inertia> for &'a AngularMomentum {
    type Output = AngularVelocity;
    fn div(self, rhs: Inertia) -> Self::Output {
        AngularVelocity::from_rad_per_second(self.as_nms() / rhs.as_kg_m2())
    }
}

impl<'a, 'b> Div<&'b Inertia> for &'a AngularMomentum {
    type Output = AngularVelocity;
    fn div(self, rhs: &'b Inertia) -> Self::Output {
        AngularVelocity::from_rad_per_second(self.as_nms() / rhs.as_kg_m2())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let j = Inertia::from_kg_m2(2.5);
        assert_eq!(j.as_kg_m2(), 2.5);
        assert_eq!(j.as_kg_mm2(), 2.5e6);
        assert_eq!(j.default_type, InertiaType::KgM2);

        // 工程常用单位：500000 kg·mm² = 0.5 kg·m²
        let j2 = Inertia::from_kg_mm2(500_000.0);
        assert_eq!(j2.as_kg_m2(), 0.5);
        assert_eq!(j2.as_kg_mm2(), 500_000.0);
        assert_eq!(j2.default_type, InertiaType::KgMm2);

        // 两个单位构造同一个物理量，互转后数值一致
        assert_eq!(Inertia::from_kg_m2(0.5).as_kg_mm2(), j2.as_kg_mm2());
    }

    #[test]
    fn test_unit_independence_in_ops() {
        // 无论用哪个单位构造，参与运算后结果必须相同
        let a = Inertia::from_kg_m2(1.0);
        let b = Inertia::from_kg_mm2(1_000_000.0);
        assert_eq!(a.as_kg_m2(), b.as_kg_m2());

        let omega = AngularVelocity::from_rad_per_second(3.0);
        assert_relative_eq!((a * omega).as_nms(), (b * omega).as_nms());

        // 加法混合单位
        let sum = a + b;
        assert_relative_eq!(sum.as_kg_m2(), 2.0);

        // 减法混合单位，结果可为负
        let diff = a - Inertia::from_kg_m2(3.0);
        assert_relative_eq!(diff.as_kg_m2(), -2.0);
    }

    #[test]
    fn test_default_and_physical_quantity() {
        let j = Inertia::default();
        assert_eq!(j.as_kg_m2(), 0.0);
        assert!(j.is_zero());
        assert_eq!(j.default_unit_value(), 0.0);
        assert_eq!(j.unit_display_name(), "Inertia (kg·m²)");

        let mut j2 = Inertia::default();
        j2.set_value(7.0);
        assert_eq!(j2.as_kg_m2(), 7.0);
    }

    #[test]
    fn test_div_inertia_gives_angular_velocity() {
        // L / J = ω：飞轮反解转速
        let l = AngularMomentum::from_nms(6.0);
        let j = Inertia::from_kg_m2(0.5);
        let omega = l / j;
        assert_relative_eq!(omega.as_rad_per_second(), 12.0);

        // 4 个引用变体
        assert_relative_eq!((l / &j).as_rad_per_second(), 12.0);
        assert_relative_eq!((&l / j).as_rad_per_second(), 12.0);
        assert_relative_eq!((&l / &j).as_rad_per_second(), 12.0);

        // 与 J·ω = L 互为逆运算
        assert_relative_eq!((j * omega).as_nms(), l.as_nms());
    }

    #[test]
    fn test_ref_variants() {
        // 加法：4 个变体（owned + 3 个引用组合）
        let a = Inertia::from_kg_m2(2.0);
        let b = Inertia::from_kg_m2(3.0);
        assert_relative_eq!((&a + &b).as_kg_m2(), 5.0);
        assert_relative_eq!((a + &b).as_kg_m2(), 5.0);
        assert_relative_eq!((&a + b).as_kg_m2(), 5.0);

        // 减法：4 个变体
        assert_relative_eq!((&b - &a).as_kg_m2(), 1.0);
        assert_relative_eq!((b - &a).as_kg_m2(), 1.0);
        assert_relative_eq!((&b - a).as_kg_m2(), 1.0);

        // Torque / AngularAcceleration 的引用变体
        let torque = Torque::from_nm(20.0);
        let alpha = AngularAcceleration::from_rad_per_second2(4.0);
        assert_relative_eq!((torque / &alpha).as_kg_m2(), 5.0);
        assert_relative_eq!((&torque / alpha).as_kg_m2(), 5.0);
        assert_relative_eq!((&torque / &alpha).as_kg_m2(), 5.0);
    }

    #[test]
    fn test_as_any_downcast() {
        // PhysicalQuantity::as_any 是向量层与泛型代码的运行时类型入口
        let j = Inertia::from_kg_m2(1.5);
        let any = j.as_any();
        assert!(any.downcast_ref::<Inertia>().is_some());
        assert_eq!(any.downcast_ref::<Inertia>().unwrap().as_kg_m2(), 1.5);
    }

    #[test]
    fn test_scalar_arith() {
        let j = Inertia::from_kg_m2(4.0);
        assert_relative_eq!((j * 2.0).as_kg_m2(), 8.0);
        assert_relative_eq!((j / 2.0).as_kg_m2(), 2.0);
        assert_relative_eq!((2.0 * j).as_kg_m2(), 8.0);
        assert_relative_eq!((j * Coef::new(0.5)).as_kg_m2(), 2.0);
        assert_relative_eq!((j / Coef::new(0.5)).as_kg_m2(), 8.0);
        assert_relative_eq!((-j).as_kg_m2(), -4.0);
        assert_relative_eq!((8.0 / j).get_value(), 2.0);
    }

    #[test]
    fn test_mul_angular_velocity_gives_angular_momentum() {
        // 飞轮：J = 0.5 kg·m²，ω = 10 rad/s  →  L = 5 kg·m²/s
        let j = Inertia::from_kg_m2(0.5);
        let omega = AngularVelocity::from_rad_per_second(10.0);
        let l = j * omega;
        assert_relative_eq!(l.as_nms(), 5.0);

        // 引用变体
        assert_relative_eq!((j * &omega).as_nms(), 5.0);
        assert_relative_eq!((&j * omega).as_nms(), 5.0);
    }

    #[test]
    fn test_div_inertia_gives_angular_acceleration() {
        // 施加 3 N·m 于 0.5 kg·m² → ω̇ = 6 rad/s²
        let torque = Torque::from_nm(3.0);
        let j = Inertia::from_kg_m2(0.5);
        let alpha = torque / j;
        assert_relative_eq!(alpha.as_rad_per_second2(), 6.0);

        assert_relative_eq!((torque / &j).as_rad_per_second2(), 6.0);
        assert_relative_eq!((&torque / j).as_rad_per_second2(), 6.0);
    }

    #[test]
    fn test_mul_angular_acceleration_gives_torque() {
        // J·ω̇ = M：0.5 kg·m² × 6 rad/s² = 3 N·m
        let j = Inertia::from_kg_m2(0.5);
        let alpha = AngularAcceleration::from_rad_per_second2(6.0);
        assert_relative_eq!((j * alpha).as_nm(), 3.0);

        assert_relative_eq!((j * &alpha).as_nm(), 3.0);
        assert_relative_eq!((&j * alpha).as_nm(), 3.0);
    }

    #[test]
    fn test_dimensional_consistency() {
        // 同一个 J 由两条路径求出，必须一致
        // 路径一：L / ω
        let omega = AngularVelocity::from_rad_per_second(4.0);
        let l = AngularMomentum::from_nms(20.0);
        let j_from_lw = l / omega;
        assert_relative_eq!(j_from_lw.as_kg_m2(), 5.0);

        // 路径二：M / ω̇ —— 与 L/ω 是同一物理量的两条独立计算路径
        let torque = Torque::from_nm(20.0);
        let alpha = AngularAcceleration::from_rad_per_second2(4.0);
        let j_from_ma = torque / alpha;
        assert_relative_eq!(j_from_ma.as_kg_m2(), 5.0);

        // 两条路径必须给出同一结果
        assert_relative_eq!(j_from_lw.as_kg_m2(), j_from_ma.as_kg_m2());

        // 闭环：J·ω 应回到原来的 L
        assert_relative_eq!((j_from_lw * omega).as_nms(), l.as_nms());

        // 闭环二：J·ω̇ 应回到施加的力矩
        let j = Inertia::from_kg_m2(5.0);
        assert_relative_eq!((j * alpha).as_nm(), torque.as_nm());
    }
}
