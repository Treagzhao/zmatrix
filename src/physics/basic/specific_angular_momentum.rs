use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, Distance, GravitationalParameter, Mass, PhysicalQuantity, SpecificAngularMomentum,
    SpecificAngularMomentumType, VectorQuantity, Velocity,
};

/// `1 km²/s` 等于多少 `m²/s`。
///
/// 1e6，不是 1e3 —— 平方公里的「平方」也在分子上。
const KM2_TO_M2: f64 = 1e6;

impl Default for SpecificAngularMomentum {
    fn default() -> Self {
        Self::from_m2_per_second(0.0)
    }
}

impl PhysicalQuantity for SpecificAngularMomentum {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_m2_per_second()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "SpecificAngularMomentum (m²/s)".to_string()
    }
}

impl VectorQuantity for SpecificAngularMomentum {}

impl SpecificAngularMomentum {
    pub fn from_m2_per_second(v: f64) -> Self {
        Self {
            default_type: SpecificAngularMomentumType::M2PerSecond,
            v,
        }
    }

    pub fn from_km2_per_second(v: f64) -> Self {
        Self {
            default_type: SpecificAngularMomentumType::Km2PerSecond,
            v,
        }
    }

    pub fn as_m2_per_second(&self) -> f64 {
        match self.default_type {
            SpecificAngularMomentumType::M2PerSecond => self.v,
            SpecificAngularMomentumType::Km2PerSecond => self.v * KM2_TO_M2,
        }
    }

    pub fn as_km2_per_second(&self) -> f64 {
        match self.default_type {
            SpecificAngularMomentumType::M2PerSecond => self.v / KM2_TO_M2,
            SpecificAngularMomentumType::Km2PerSecond => self.v,
        }
    }
}

impl Add for SpecificAngularMomentum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() + rhs.as_m2_per_second())
    }
}

impl<'a, 'b> Add<&'b SpecificAngularMomentum> for &'a SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn add(self, rhs: &'b SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() + rhs.as_m2_per_second(),
        )
    }
}
impl<'a> Add<&'a SpecificAngularMomentum> for SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn add(self, rhs: &'a SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() + rhs.as_m2_per_second(),
        )
    }
}
impl<'a> Add<SpecificAngularMomentum> for &'a SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn add(self, rhs: SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() + rhs.as_m2_per_second(),
        )
    }
}

impl Sub for SpecificAngularMomentum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() - rhs.as_m2_per_second())
    }
}

impl<'a, 'b> Sub<&'b SpecificAngularMomentum> for &'a SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn sub(self, rhs: &'b SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() - rhs.as_m2_per_second(),
        )
    }
}
impl<'a> Sub<&'a SpecificAngularMomentum> for SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn sub(self, rhs: &'a SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() - rhs.as_m2_per_second(),
        )
    }
}
impl<'a> Sub<SpecificAngularMomentum> for &'a SpecificAngularMomentum {
    type Output = SpecificAngularMomentum;
    fn sub(self, rhs: SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_m2_per_second() - rhs.as_m2_per_second(),
        )
    }
}

impl Neg for SpecificAngularMomentum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_m2_per_second(-self.as_m2_per_second())
    }
}

impl Mul<f64> for SpecificAngularMomentum {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() * rhs)
    }
}

impl Div<f64> for SpecificAngularMomentum {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() / rhs)
    }
}

impl Mul<SpecificAngularMomentum> for f64 {
    type Output = SpecificAngularMomentum;
    fn mul(self, rhs: SpecificAngularMomentum) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(self * rhs.as_m2_per_second())
    }
}

impl Mul<Coef> for SpecificAngularMomentum {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() * rhs.get_value())
    }
}

impl Div<Coef> for SpecificAngularMomentum {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_m2_per_second(self.as_m2_per_second() / rhs.get_value())
    }
}

// 比角动量 ÷ 距离 = 速度：v = h/r
impl Div<Distance> for SpecificAngularMomentum {
    type Output = Velocity;
    fn div(self, rhs: Distance) -> Self::Output {
        Velocity::from_m_per_sec(self.as_m2_per_second() / rhs.as_m())
    }
}

impl Div<Distance> for &SpecificAngularMomentum {
    type Output = Velocity;
    fn div(self, rhs: Distance) -> Self::Output {
        Velocity::from_m_per_sec(self.as_m2_per_second() / rhs.as_m())
    }
}

impl<'a> Div<&'a Distance> for SpecificAngularMomentum {
    type Output = Velocity;
    fn div(self, rhs: &'a Distance) -> Self::Output {
        Velocity::from_m_per_sec(self.as_m2_per_second() / rhs.as_m())
    }
}

impl<'a, 'b> Div<&'b Distance> for &'a SpecificAngularMomentum {
    type Output = Velocity;
    fn div(self, rhs: &'b Distance) -> Self::Output {
        Velocity::from_m_per_sec(self.as_m2_per_second() / rhs.as_m())
    }
}

// 速度 × 距离 = 比角动量（标量形式，只对垂直情形成立）
impl Mul<Distance> for Velocity {
    type Output = SpecificAngularMomentum;
    fn mul(self, rhs: Distance) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(self.as_m_per_sec() * rhs.as_m())
    }
}

impl Mul<Velocity> for Distance {
    type Output = SpecificAngularMomentum;
    fn mul(self, rhs: Velocity) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(self.as_m() * rhs.as_m_per_sec())
    }
}

// 角动量 ÷ 质量 = 比角动量：h = L/m
impl Div<Mass> for crate::physics::basic::AngularMomentum {
    type Output = SpecificAngularMomentum;
    fn div(self, rhs: Mass) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_kg_m2_per_second() / rhs.as_kg(),
        )
    }
}

impl Div<Mass> for &crate::physics::basic::AngularMomentum {
    type Output = SpecificAngularMomentum;
    fn div(self, rhs: Mass) -> Self::Output {
        SpecificAngularMomentum::from_m2_per_second(
            self.as_kg_m2_per_second() / rhs.as_kg(),
        )
    }
}

// 比角动量 × 质量 = 角动量
impl Mul<Mass> for SpecificAngularMomentum {
    type Output = crate::physics::basic::AngularMomentum;
    fn mul(self, rhs: Mass) -> Self::Output {
        crate::physics::basic::AngularMomentum::from_kg_m2_per_second(
            self.as_m2_per_second() * rhs.as_kg(),
        )
    }
}

// 比角动量 × 引力参数 = ？（m⁵/s⁴，无物理意义，不提供）
//
// 但 比角动量² = m⁴/s²，÷ 引力参数 = m。这个链条在轨道计算里常用，
// 所以提供命名方法而不是运算符：
impl SpecificAngularMomentum {
    /// 半通径：`p = h² / μ`。
    ///
    /// **这是半通径（semi-latus rectum），不是半长轴。**
    /// 关系是 `p = a(1 − e²)`，所以 `a = p / (1 − e²)`。
    ///
    /// # 为什么用方法而不是运算符
    ///
    /// `h * h / mu` 能算出来，但中间量 `h²` 的量纲是 `m⁴/s²`——
    /// 这个量在物理上只做一件事就是除以 `μ`，为它单独建类型不值得。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{GravitationalParameter, SpecificAngularMomentum};
    ///
    /// let earth = GravitationalParameter::from_m3_per_s2(3.986004418e14);
    /// // 700 km 圆轨道：h = r·v = 7078e3 × 7507 ≈ 5.313e10
    /// let h = SpecificAngularMomentum::from_m2_per_second(5.313e10);
    /// let p = h.semi_latus_rectum(earth);
    /// assert!((p.as_m() - 7.083e6).abs() < 1e4);
    /// ```
    pub fn semi_latus_rectum(&self, mu: GravitationalParameter) -> Distance {
        let h = self.as_m2_per_second();
        Distance::from_m(h * h / mu.as_m3_per_s2())
    }

    /// 半长轴：`a = p / (1 − e²)`，其中 `p = h²/μ`。
    ///
    /// 抛物线轨道（`e = 1`）时 `a` 无定义，返回 `None`。
    /// **不返回 `inf`**：无穷大混进后续计算会静默污染结果，
    /// 而 `Option` 强制调用方处理这个边界。
    ///
    /// 双曲线（`e > 1`）时 `1 − e² < 0`，得到负的 `a`。
    /// 这是双曲线轨道的约定（`a < 0` 表示不闭合），不是错误，所以照常返回。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{Coef, GravitationalParameter, SpecificAngularMomentum};
    ///
    /// let earth = GravitationalParameter::from_m3_per_s2(3.986004418e14);
    /// let h = SpecificAngularMomentum::from_m2_per_second(5.313e10);
    ///
    /// // 圆轨道 e=0：a = p
    /// let p = h.semi_latus_rectum(earth);
    /// let a = h.semi_major_axis(earth, Coef::new(0.0)).unwrap();
    /// assert!((a.as_m() - p.as_m()).abs() < 1e-6);
    ///
    /// // 抛物线 e=1：a 无定义
    /// assert!(h.semi_major_axis(earth, Coef::new(1.0)).is_none());
    /// ```
    pub fn semi_major_axis(
        &self,
        mu: GravitationalParameter,
        eccentricity: Coef,
    ) -> Option<Distance> {
        let p = self.semi_latus_rectum(mu).as_m();
        let e = eccentricity.get_value();
        let denom = 1.0 - e * e;
        if denom.abs() < 1e-15 {
            return None;
        }
        Some(Distance::from_m(p / denom))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let h = SpecificAngularMomentum::from_m2_per_second(1e6);
        assert_relative_eq!(h.as_m2_per_second(), 1e6, epsilon = 1e-6);
        assert_relative_eq!(h.as_km2_per_second(), 1.0, epsilon = 1e-12);

        let h = SpecificAngularMomentum::from_km2_per_second(1.0);
        assert_relative_eq!(h.as_m2_per_second(), 1e6, epsilon = 1e-6);
    }

    #[test]
    fn test_km2_factor_is_1e6_not_1e3() {
        // 最易错处：平方公里的「平方」也在分子上，因子是 1e6
        let km = SpecificAngularMomentum::from_km2_per_second(1.0);
        assert_relative_eq!(km.as_m2_per_second(), 1e6, epsilon = 1e-6);
        assert!((km.as_m2_per_second() - 1e3).abs() > 1.0, "不是 1e3");
    }

    #[test]
    fn test_all_unit_branches() {
        let a = SpecificAngularMomentum::from_m2_per_second(5.9e10);
        let b = SpecificAngularMomentum::from_km2_per_second(5.9e4);

        for h in [a, b] {
            assert_relative_eq!(h.as_m2_per_second(), 5.9e10, epsilon = 1e4);
            assert_relative_eq!(h.as_km2_per_second(), 5.9e4, epsilon = 1e-8);
        }
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 1.0, 5.9e10, 1e15] {
            let h = SpecificAngularMomentum::from_m2_per_second(v);
            assert_relative_eq!(
                SpecificAngularMomentum::from_km2_per_second(h.as_km2_per_second())
                    .as_m2_per_second(),
                v,
                epsilon = v.abs() * 1e-12 + 1e-12
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = SpecificAngularMomentum::from_m2_per_second(10.0);
        let b = SpecificAngularMomentum::from_m2_per_second(4.0);

        assert_relative_eq!((a + b).as_m2_per_second(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + &b).as_m2_per_second(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((a + &b).as_m2_per_second(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + b).as_m2_per_second(), 14.0, epsilon = 1e-12);

        assert_relative_eq!((a - b).as_m2_per_second(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - &b).as_m2_per_second(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((a - &b).as_m2_per_second(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - b).as_m2_per_second(), 6.0, epsilon = 1e-12);

        assert_relative_eq!((a * 2.0).as_m2_per_second(), 20.0, epsilon = 1e-12);
        assert_relative_eq!((a / 4.0).as_m2_per_second(), 2.5, epsilon = 1e-12);
        assert_relative_eq!((0.5 * a).as_m2_per_second(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((a * Coef::new(3.0)).as_m2_per_second(), 30.0, epsilon = 1e-12);
        assert_relative_eq!((a / Coef::new(2.0)).as_m2_per_second(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((-a).as_m2_per_second(), -10.0, epsilon = 1e-12);
    }

    #[test]
    fn test_div_distance_gives_velocity() {
        // h/r = v：圆轨道 7000 km，h = 7000e3 × 7546
        let h = SpecificAngularMomentum::from_m2_per_second(7000e3 * 7546.0);
        let r = Distance::from_m(7000e3);
        assert_relative_eq!(h.div(r).as_m_per_sec(), 7546.0, epsilon = 1e-9);
    }

    #[test]
    fn test_div_distance_all_ref_variants() {
        // 4 个引用变体都要覆盖
        let h = SpecificAngularMomentum::from_m2_per_second(7000e3 * 7546.0);
        let r = Distance::from_m(7000e3);

        assert_relative_eq!((h / r).as_m_per_sec(), 7546.0, epsilon = 1e-9);
        assert_relative_eq!((&h / r).as_m_per_sec(), 7546.0, epsilon = 1e-9);
        assert_relative_eq!((h / &r).as_m_per_sec(), 7546.0, epsilon = 1e-9);
        assert_relative_eq!((&h / &r).as_m_per_sec(), 7546.0, epsilon = 1e-9);
    }

    #[test]
    fn test_scalar_mul_distance_gives_h() {
        // v × r = h（标量，仅垂直情形）
        let v = Velocity::from_m_per_sec(7546.0);
        let r = Distance::from_m(7000e3);
        assert_relative_eq!((v * r).as_m2_per_second(), 7000e3 * 7546.0, epsilon = 1e-6);
        assert_relative_eq!((r * v).as_m2_per_second(), 7000e3 * 7546.0, epsilon = 1e-6);
    }

    #[test]
    fn test_angular_momentum_div_mass_gives_specific() {
        // h = L/m
        use crate::physics::basic::AngularMomentum;
        let l = AngularMomentum::from_kg_m2_per_second(7000e3 * 7546.0 * 500.0);
        let m = Mass::from_kg(500.0);
        assert_relative_eq!((l / m).as_m2_per_second(), 7000e3 * 7546.0, epsilon = 1e-6);
        assert_relative_eq!((&l / m).as_m2_per_second(), 7000e3 * 7546.0, epsilon = 1e-6);
    }

    #[test]
    fn test_specific_mul_mass_gives_angular_momentum() {
        let h = SpecificAngularMomentum::from_m2_per_second(7000e3 * 7546.0);
        let l = h * Mass::from_kg(500.0);
        assert_relative_eq!(
            l.as_kg_m2_per_second(),
            7000e3 * 7546.0 * 500.0,
            epsilon = 1e-2
        );
    }

    #[test]
    fn test_semi_latus_rectum() {
        // 地球 700 km 圆轨道：r = 7078e3, v = 7507 m/s
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let h = SpecificAngularMomentum::from_m2_per_second(7078e3 * 7507.0);
        let p = h.semi_latus_rectum(mu);
        // 圆轨道 p = a = r
        assert_relative_eq!(p.as_m(), 7078e3, epsilon = 1e4);
    }

    #[test]
    fn test_semi_major_axis_circular() {
        // e=0：a = p
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let h = SpecificAngularMomentum::from_m2_per_second(7078e3 * 7507.0);
        let p = h.semi_latus_rectum(mu);
        let a = h.semi_major_axis(mu, Coef::new(0.0)).unwrap();
        assert_relative_eq!(a.as_m(), p.as_m(), epsilon = 1e-6);
    }

    #[test]
    fn test_semi_major_axis_elliptic() {
        // a = 7000 km, e = 0.1 → p = a(1-e²) = 6930 km
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let a_true = 7000e3;
        let e = 0.1;
        let p_true = a_true * (1.0 - e * e);
        // 由 p 反算 h
        let h_true = (mu.as_m3_per_s2() * p_true).sqrt();
        let h = SpecificAngularMomentum::from_m2_per_second(h_true);

        let p = h.semi_latus_rectum(mu);
        assert_relative_eq!(p.as_m(), p_true, epsilon = 1.0);

        let a = h.semi_major_axis(mu, Coef::new(e)).unwrap();
        assert_relative_eq!(a.as_m(), a_true, epsilon = 1.0);
    }

    #[test]
    fn test_semi_major_axis_parabolic_is_none() {
        // e=1：a 无定义
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let h = SpecificAngularMomentum::from_m2_per_second(5.9e10);
        assert!(h.semi_major_axis(mu, Coef::new(1.0)).is_none());
        assert!(h.semi_major_axis(mu, Coef::new(-1.0)).is_none());
    }

    #[test]
    fn test_semi_major_axis_hyperbolic_is_negative() {
        // e > 1：a < 0，双曲线约定
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let h = SpecificAngularMomentum::from_m2_per_second(5.9e10);
        let a = h.semi_major_axis(mu, Coef::new(2.0)).unwrap();
        assert!(a.as_m() < 0.0, "双曲线 a 应为负");
    }

    #[test]
    fn test_wrong_formula_would_be_wrong_by_1_minus_e_squared() {
        // 本测试钉住「乘 (1-e²)」是错的：那等于 a(1−e²)²，而不只是差 (1−e²) 倍
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let a_true = 7000e3;
        let e = 0.1;
        let p = a_true * (1.0 - e * e);
        let h = SpecificAngularMomentum::from_m2_per_second((mu.as_m3_per_s2() * p).sqrt());

        let correct = h.semi_major_axis(mu, Coef::new(e)).unwrap().as_m();
        let wrong = h.semi_latus_rectum(mu).as_m() * (1.0 - e * e);

        assert_relative_eq!(correct, a_true, epsilon = 1.0);

        // wrong = a(1−e²)²，而 correct = a，所以比值是 1/(1−e²)²
        let factor = 1.0 - e * e;
        assert_relative_eq!(correct / wrong, 1.0 / (factor * factor), epsilon = 1e-9);

        // 具体数值：e=0.1 时偏小 1.99%
        assert_relative_eq!(wrong / correct, 0.9801, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let h = SpecificAngularMomentum::default();
        assert!(h.is_zero());
        assert_eq!(h.default_unit_value(), 0.0);
        assert_eq!(h.unit_display_name(), "SpecificAngularMomentum (m²/s)");

        let mut h2 = SpecificAngularMomentum::default();
        h2.set_value(5.9e10);
        assert_relative_eq!(h2.as_m2_per_second(), 5.9e10, epsilon = 1e-4);
        assert!(!h2.is_zero());

        assert!(h2.as_any().downcast_ref::<SpecificAngularMomentum>().is_some());
    }
}
