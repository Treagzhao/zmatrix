use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, GravitationalParameter, GravitationalParameterType, PhysicalQuantity,
};

/// `1 km³/s²` 等于多少 `m³/s²`。
///
/// 1e9 —— 立方公里的「立方」在分子上。地球 μ 用 km 单位是 3.986e5，
/// 用 m 单位是 3.986e14，差 9 个数量级。
const KM3_TO_M3: f64 = 1e9;

impl Default for GravitationalParameter {
    fn default() -> Self {
        Self::from_m3_per_s2(0.0)
    }
}

impl PhysicalQuantity for GravitationalParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_m3_per_s2()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "GravitationalParameter (m³/s²)".to_string()
    }
}

impl GravitationalParameter {
    pub fn from_m3_per_s2(v: f64) -> Self {
        Self {
            default_type: GravitationalParameterType::M3PerSecondSquared,
            v,
        }
    }

    pub fn from_km3_per_s2(v: f64) -> Self {
        Self {
            default_type: GravitationalParameterType::Km3PerSecondSquared,
            v,
        }
    }

    pub fn as_m3_per_s2(&self) -> f64 {
        match self.default_type {
            GravitationalParameterType::M3PerSecondSquared => self.v,
            GravitationalParameterType::Km3PerSecondSquared => self.v * KM3_TO_M3,
        }
    }

    pub fn as_km3_per_s2(&self) -> f64 {
        match self.default_type {
            GravitationalParameterType::M3PerSecondSquared => self.v / KM3_TO_M3,
            GravitationalParameterType::Km3PerSecondSquared => self.v,
        }
    }

    /// 地球引力参数（WGS-84 值）：`3.986004418e14 m³/s²`。
    pub fn earth() -> Self {
        Self::from_m3_per_s2(3.986_004_418e14)
    }

    /// 月球引力参数：`4.9048695e12 m³/s²`。
    pub fn moon() -> Self {
        Self::from_m3_per_s2(4.904_869_5e12)
    }

    /// 太阳引力参数：`1.32712440018e20 m³/s²`。
    pub fn sun() -> Self {
        Self::from_m3_per_s2(1.327_124_400_18e20)
    }
}

impl Add for GravitationalParameter {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() + rhs.as_m3_per_s2())
    }
}

impl<'a, 'b> Add<&'b GravitationalParameter> for &'a GravitationalParameter {
    type Output = GravitationalParameter;
    fn add(self, rhs: &'b GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() + rhs.as_m3_per_s2())
    }
}
impl<'a> Add<&'a GravitationalParameter> for GravitationalParameter {
    type Output = GravitationalParameter;
    fn add(self, rhs: &'a GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() + rhs.as_m3_per_s2())
    }
}
impl<'a> Add<GravitationalParameter> for &'a GravitationalParameter {
    type Output = GravitationalParameter;
    fn add(self, rhs: GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() + rhs.as_m3_per_s2())
    }
}

impl Sub for GravitationalParameter {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() - rhs.as_m3_per_s2())
    }
}

impl<'a, 'b> Sub<&'b GravitationalParameter> for &'a GravitationalParameter {
    type Output = GravitationalParameter;
    fn sub(self, rhs: &'b GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() - rhs.as_m3_per_s2())
    }
}
impl<'a> Sub<&'a GravitationalParameter> for GravitationalParameter {
    type Output = GravitationalParameter;
    fn sub(self, rhs: &'a GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() - rhs.as_m3_per_s2())
    }
}
impl<'a> Sub<GravitationalParameter> for &'a GravitationalParameter {
    type Output = GravitationalParameter;
    fn sub(self, rhs: GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self.as_m3_per_s2() - rhs.as_m3_per_s2())
    }
}

impl Neg for GravitationalParameter {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_m3_per_s2(-self.as_m3_per_s2())
    }
}

impl Mul<f64> for GravitationalParameter {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() * rhs)
    }
}

impl Div<f64> for GravitationalParameter {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() / rhs)
    }
}

impl Mul<GravitationalParameter> for f64 {
    type Output = GravitationalParameter;
    fn mul(self, rhs: GravitationalParameter) -> Self::Output {
        GravitationalParameter::from_m3_per_s2(self * rhs.as_m3_per_s2())
    }
}

impl Mul<Coef> for GravitationalParameter {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() * rhs.get_value())
    }
}

impl Div<Coef> for GravitationalParameter {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_m3_per_s2(self.as_m3_per_s2() / rhs.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let mu = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        assert_relative_eq!(mu.as_m3_per_s2(), 3.986004418e14, epsilon = 1e4);
        assert_relative_eq!(mu.as_km3_per_s2(), 3.986004418e5, epsilon = 1e-5);

        let mu = GravitationalParameter::from_km3_per_s2(398600.4418);
        assert_relative_eq!(mu.as_m3_per_s2(), 3.986004418e14, epsilon = 1e4);
    }

    #[test]
    fn test_km3_factor_is_1e9() {
        // 立方公里的因子是 1e9，不是 1e3 也不是 1e6
        let km = GravitationalParameter::from_km3_per_s2(1.0);
        assert_relative_eq!(km.as_m3_per_s2(), 1e9, epsilon = 1e-3);
    }

    #[test]
    fn test_builtin_constants() {
        // 地球
        assert_relative_eq!(
            GravitationalParameter::earth().as_m3_per_s2(),
            3.986004418e14,
            epsilon = 1e4
        );
        // 月球
        assert_relative_eq!(
            GravitationalParameter::moon().as_m3_per_s2(),
            4.9048695e12,
            epsilon = 1e2
        );
        // 太阳
        assert_relative_eq!(
            GravitationalParameter::sun().as_m3_per_s2(),
            1.32712440018e20,
            epsilon = 1e10
        );
    }

    #[test]
    fn test_all_unit_branches() {
        let a = GravitationalParameter::from_m3_per_s2(3.986004418e14);
        let b = GravitationalParameter::from_km3_per_s2(3.986004418e5);

        for mu in [a, b] {
            assert_relative_eq!(mu.as_m3_per_s2(), 3.986004418e14, epsilon = 1e4);
            assert_relative_eq!(mu.as_km3_per_s2(), 3.986004418e5, epsilon = 1e-5);
        }
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 1.0, 3.986004418e14, 1.32712440018e20] {
            let mu = GravitationalParameter::from_m3_per_s2(v);
            assert_relative_eq!(
                GravitationalParameter::from_km3_per_s2(mu.as_km3_per_s2()).as_m3_per_s2(),
                v,
                epsilon = v.abs() * 1e-12 + 1e-9
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = GravitationalParameter::from_m3_per_s2(10.0);
        let b = GravitationalParameter::from_m3_per_s2(4.0);

        assert_relative_eq!((a + b).as_m3_per_s2(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + &b).as_m3_per_s2(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((a + &b).as_m3_per_s2(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + b).as_m3_per_s2(), 14.0, epsilon = 1e-12);

        assert_relative_eq!((a - b).as_m3_per_s2(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - &b).as_m3_per_s2(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((a - &b).as_m3_per_s2(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - b).as_m3_per_s2(), 6.0, epsilon = 1e-12);

        assert_relative_eq!((a * 2.0).as_m3_per_s2(), 20.0, epsilon = 1e-12);
        assert_relative_eq!((a / 4.0).as_m3_per_s2(), 2.5, epsilon = 1e-12);
        assert_relative_eq!((0.5 * a).as_m3_per_s2(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((a * Coef::new(3.0)).as_m3_per_s2(), 30.0, epsilon = 1e-12);
        assert_relative_eq!((a / Coef::new(2.0)).as_m3_per_s2(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((-a).as_m3_per_s2(), -10.0, epsilon = 1e-12);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let mu = GravitationalParameter::default();
        assert!(mu.is_zero());
        assert_eq!(mu.default_unit_value(), 0.0);
        assert_eq!(mu.unit_display_name(), "GravitationalParameter (m³/s²)");

        let mut mu2 = GravitationalParameter::default();
        mu2.set_value(3.986e14);
        assert_relative_eq!(mu2.as_m3_per_s2(), 3.986e14, epsilon = 1e4);
        assert!(!mu2.is_zero());

        assert!(mu2.as_any().downcast_ref::<GravitationalParameter>().is_some());
    }
}
