use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, Density, Mass, MassFlowRate, MassFlowRateType, MassFlux, PhysicalQuantity, Velocity,
};

/// 一秒有多少小时。
const SECONDS_PER_HOUR: f64 = 3600.0;
/// `1 g/s` 等于多少 `kg/s`。
const G_PER_S_TO_KG: f64 = 1e-3;
/// `1 t/h` 等于多少 `kg/s`。
///
/// 1000/3600，不是 1/3600 —— 吨比千克大 1000 倍，小时比秒大 3600 倍，
/// 两个因子方向相反，净结果是 0.27778。搞混会差 1e6。
const T_PER_H_TO_KG_PER_S: f64 = 1000.0 / SECONDS_PER_HOUR;
/// `1 kg/h` 等于多少 `kg/s`。
const KG_PER_H_TO_KG_PER_S: f64 = 1.0 / SECONDS_PER_HOUR;

impl Default for MassFlowRate {
    fn default() -> Self {
        Self::from_kg_per_s(0.0)
    }
}

impl PhysicalQuantity for MassFlowRate {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_per_s()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "MassFlowRate (kg/s)".to_string()
    }
}

impl MassFlowRate {
    pub fn from_kg_per_s(v: f64) -> Self {
        Self {
            default_type: MassFlowRateType::KilogramPerSecond,
            v,
        }
    }

    pub fn from_g_per_s(v: f64) -> Self {
        Self {
            default_type: MassFlowRateType::GramPerSecond,
            v,
        }
    }

    pub fn from_kg_per_hour(v: f64) -> Self {
        Self {
            default_type: MassFlowRateType::KilogramPerHour,
            v,
        }
    }

    pub fn from_ton_per_hour(v: f64) -> Self {
        Self {
            default_type: MassFlowRateType::TonPerHour,
            v,
        }
    }

    pub fn as_kg_per_s(&self) -> f64 {
        match self.default_type {
            MassFlowRateType::KilogramPerSecond => self.v,
            MassFlowRateType::GramPerSecond => self.v * G_PER_S_TO_KG,
            MassFlowRateType::KilogramPerHour => self.v * KG_PER_H_TO_KG_PER_S,
            MassFlowRateType::TonPerHour => self.v * T_PER_H_TO_KG_PER_S,
        }
    }

    pub fn as_g_per_s(&self) -> f64 {
        self.as_kg_per_s() / G_PER_S_TO_KG
    }

    pub fn as_kg_per_hour(&self) -> f64 {
        self.as_kg_per_s() / KG_PER_H_TO_KG_PER_S
    }

    pub fn as_ton_per_hour(&self) -> f64 {
        self.as_kg_per_s() / T_PER_H_TO_KG_PER_S
    }

    /// 由通量与截面反推平均速度：`v = ṁ / (ρ · A)`。
    ///
    /// 需要同时给出密度——流量本身不含密度信息。
    pub fn velocity(&self, density: Density, area: crate::physics::basic::Area) -> Velocity {
        let denom = density.as_kg_per_m3() * area.as_m2();
        Velocity::from_m_per_sec(self.as_kg_per_s() / denom)
    }
}

impl Add for MassFlowRate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() + rhs.as_kg_per_s())
    }
}

impl<'a, 'b> Add<&'b MassFlowRate> for &'a MassFlowRate {
    type Output = MassFlowRate;
    fn add(self, rhs: &'b MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() + rhs.as_kg_per_s())
    }
}
impl<'a> Add<&'a MassFlowRate> for MassFlowRate {
    type Output = MassFlowRate;
    fn add(self, rhs: &'a MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() + rhs.as_kg_per_s())
    }
}
impl<'a> Add<MassFlowRate> for &'a MassFlowRate {
    type Output = MassFlowRate;
    fn add(self, rhs: MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() + rhs.as_kg_per_s())
    }
}

impl Sub for MassFlowRate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() - rhs.as_kg_per_s())
    }
}

impl<'a, 'b> Sub<&'b MassFlowRate> for &'a MassFlowRate {
    type Output = MassFlowRate;
    fn sub(self, rhs: &'b MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() - rhs.as_kg_per_s())
    }
}
impl<'a> Sub<&'a MassFlowRate> for MassFlowRate {
    type Output = MassFlowRate;
    fn sub(self, rhs: &'a MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() - rhs.as_kg_per_s())
    }
}
impl<'a> Sub<MassFlowRate> for &'a MassFlowRate {
    type Output = MassFlowRate;
    fn sub(self, rhs: MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg_per_s() - rhs.as_kg_per_s())
    }
}

impl Neg for MassFlowRate {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_kg_per_s(-self.as_kg_per_s())
    }
}

impl Mul<f64> for MassFlowRate {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() * rhs)
    }
}

impl Div<f64> for MassFlowRate {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() / rhs)
    }
}

impl Mul<MassFlowRate> for f64 {
    type Output = MassFlowRate;
    fn mul(self, rhs: MassFlowRate) -> Self::Output {
        MassFlowRate::from_kg_per_s(self * rhs.as_kg_per_s())
    }
}

impl Mul<Coef> for MassFlowRate {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() * rhs.get_value())
    }
}

impl Div<Coef> for MassFlowRate {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_s(self.as_kg_per_s() / rhs.get_value())
    }
}

// 质量流量 × 时间 = 质量：m = ṁ·Δt
impl Mul<chrono::TimeDelta> for MassFlowRate {
    type Output = Mass;
    fn mul(self, rhs: chrono::TimeDelta) -> Self::Output {
        Mass::from_kg(self.as_kg_per_s() * rhs.as_seconds_f64())
    }
}

impl Mul<chrono::TimeDelta> for &MassFlowRate {
    type Output = Mass;
    fn mul(self, rhs: chrono::TimeDelta) -> Self::Output {
        Mass::from_kg(self.as_kg_per_s() * rhs.as_seconds_f64())
    }
}

impl<'a> Mul<&'a chrono::TimeDelta> for MassFlowRate {
    type Output = Mass;
    fn mul(self, rhs: &'a chrono::TimeDelta) -> Self::Output {
        Mass::from_kg(self.as_kg_per_s() * rhs.as_seconds_f64())
    }
}

impl<'a, 'b> Mul<&'b chrono::TimeDelta> for &'a MassFlowRate {
    type Output = Mass;
    fn mul(self, rhs: &'b chrono::TimeDelta) -> Self::Output {
        Mass::from_kg(self.as_kg_per_s() * rhs.as_seconds_f64())
    }
}

// 时间 × 质量流量 = 质量（交换律）
impl Mul<MassFlowRate> for chrono::TimeDelta {
    type Output = Mass;
    fn mul(self, rhs: MassFlowRate) -> Self::Output {
        Mass::from_kg(self.as_seconds_f64() * rhs.as_kg_per_s())
    }
}

// 质量 ÷ 时间 = 质量流量：ṁ = m/Δt
impl Div<chrono::TimeDelta> for Mass {
    type Output = MassFlowRate;
    fn div(self, rhs: chrono::TimeDelta) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg() / rhs.as_seconds_f64())
    }
}

impl Div<chrono::TimeDelta> for &Mass {
    type Output = MassFlowRate;
    fn div(self, rhs: chrono::TimeDelta) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg() / rhs.as_seconds_f64())
    }
}

impl<'a> Div<&'a chrono::TimeDelta> for Mass {
    type Output = MassFlowRate;
    fn div(self, rhs: &'a chrono::TimeDelta) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg() / rhs.as_seconds_f64())
    }
}

impl<'a, 'b> Div<&'b chrono::TimeDelta> for &'a Mass {
    type Output = MassFlowRate;
    fn div(self, rhs: &'b chrono::TimeDelta) -> Self::Output {
        MassFlowRate::from_kg_per_s(self.as_kg() / rhs.as_seconds_f64())
    }
}

// 质量流量 ÷ 通量 = 面积：A = ṁ/q
impl Div<MassFlux> for MassFlowRate {
    type Output = crate::physics::basic::Area;
    fn div(self, rhs: MassFlux) -> Self::Output {
        crate::physics::basic::Area::from_m2(self.as_kg_per_s() / rhs.as_kg_per_m2_s())
    }
}

impl Div<MassFlux> for &MassFlowRate {
    type Output = crate::physics::basic::Area;
    fn div(self, rhs: MassFlux) -> Self::Output {
        crate::physics::basic::Area::from_m2(self.as_kg_per_s() / rhs.as_kg_per_m2_s())
    }
}

impl<'a> Div<&'a MassFlux> for MassFlowRate {
    type Output = crate::physics::basic::Area;
    fn div(self, rhs: &'a MassFlux) -> Self::Output {
        crate::physics::basic::Area::from_m2(self.as_kg_per_s() / rhs.as_kg_per_m2_s())
    }
}

impl<'a, 'b> Div<&'b MassFlux> for &'a MassFlowRate {
    type Output = crate::physics::basic::Area;
    fn div(self, rhs: &'b MassFlux) -> Self::Output {
        crate::physics::basic::Area::from_m2(self.as_kg_per_s() / rhs.as_kg_per_m2_s())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::Area;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        assert_relative_eq!(MassFlowRate::from_kg_per_s(1.0).as_g_per_s(), 1000.0, epsilon = 1e-9);
        assert_relative_eq!(MassFlowRate::from_g_per_s(1000.0).as_kg_per_s(), 1.0, epsilon = 1e-12);

        // 1 kg/h = 1/3600 kg/s
        assert_relative_eq!(
            MassFlowRate::from_kg_per_hour(3600.0).as_kg_per_s(),
            1.0,
            epsilon = 1e-12
        );

        // 1 t/h = 1000/3600 = 0.27778 kg/s
        assert_relative_eq!(
            MassFlowRate::from_ton_per_hour(1.0).as_kg_per_s(),
            1000.0 / 3600.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_ton_per_hour_factor() {
        // 最易错处：t/h → kg/s 的因子是 0.27778，不是 1/3600 也不是 3600
        let t = MassFlowRate::from_ton_per_hour(1.0);
        assert_relative_eq!(t.as_kg_per_s(), 0.277_777_777, epsilon = 1e-6);

        // 1 t/h = 1000 kg/h
        assert_relative_eq!(t.as_kg_per_hour(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 同一流量四种表示
        let a = MassFlowRate::from_kg_per_s(1.0);
        let b = MassFlowRate::from_g_per_s(1000.0);
        let c = MassFlowRate::from_kg_per_hour(3600.0);
        let d = MassFlowRate::from_ton_per_hour(3.6); // 3.6 t/h = 1 kg/s

        for r in [a, b, c, d] {
            assert_relative_eq!(r.as_kg_per_s(), 1.0, epsilon = 1e-9);
            assert_relative_eq!(r.as_g_per_s(), 1000.0, epsilon = 1e-6);
            assert_relative_eq!(r.as_kg_per_hour(), 3600.0, epsilon = 1e-6);
            assert_relative_eq!(r.as_ton_per_hour(), 3.6, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 1e-6, 0.5, 122.5, 1e4] {
            let r = MassFlowRate::from_kg_per_s(v);
            assert_relative_eq!(
                MassFlowRate::from_g_per_s(r.as_g_per_s()).as_kg_per_s(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                MassFlowRate::from_kg_per_hour(r.as_kg_per_hour()).as_kg_per_s(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                MassFlowRate::from_ton_per_hour(r.as_ton_per_hour()).as_kg_per_s(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = MassFlowRate::from_kg_per_s(10.0);
        let b = MassFlowRate::from_g_per_s(10_000.0); // 同为 10 kg/s

        assert_relative_eq!((a + b).as_kg_per_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_kg_per_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_kg_per_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_kg_per_s(), 20.0, epsilon = 1e-9);

        assert_relative_eq!((a - b).as_kg_per_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_kg_per_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_kg_per_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_kg_per_s(), 0.0, epsilon = 1e-9);

        assert_relative_eq!((a * 2.0).as_kg_per_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((a / 4.0).as_kg_per_s(), 2.5, epsilon = 1e-9);
        assert_relative_eq!((0.5 * a).as_kg_per_s(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((a * Coef::new(3.0)).as_kg_per_s(), 30.0, epsilon = 1e-9);
        assert_relative_eq!((a / Coef::new(2.0)).as_kg_per_s(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((-a).as_kg_per_s(), -10.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mul_time_gives_mass() {
        let rate = MassFlowRate::from_kg_per_s(122.5);
        let dt = chrono::TimeDelta::seconds(10);

        assert_relative_eq!((rate * dt).as_kg(), 1225.0, epsilon = 1e-9);
        assert_relative_eq!((&rate * dt).as_kg(), 1225.0, epsilon = 1e-9);
        assert_relative_eq!((rate * &dt).as_kg(), 1225.0, epsilon = 1e-9);
        assert_relative_eq!((&rate * &dt).as_kg(), 1225.0, epsilon = 1e-9);

        // 交换律
        assert_relative_eq!((dt * rate).as_kg(), 1225.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mass_div_time_gives_rate() {
        let m = Mass::from_kg(1225.0);
        let dt = chrono::TimeDelta::seconds(10);
        let rate = m / dt;
        assert_relative_eq!(rate.as_kg_per_s(), 122.5, epsilon = 1e-9);
        assert_relative_eq!((&m / &dt).as_kg_per_s(), 122.5, epsilon = 1e-9);
        assert_relative_eq!((m / &dt).as_kg_per_s(), 122.5, epsilon = 1e-9);
        assert_relative_eq!((&m / dt).as_kg_per_s(), 122.5, epsilon = 1e-9);
    }

    #[test]
    fn test_rate_div_flux_gives_area() {
        let rate = MassFlowRate::from_kg_per_s(122.5);
        let flux = MassFlux::from_kg_per_m2_s(245.0);
        let area = rate / flux;
        assert_relative_eq!(area.as_m2(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((&rate / &flux).as_m2(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((rate / &flux).as_m2(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((&rate / flux).as_m2(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn test_velocity_from_rate_density_area() {
        // v = ṁ/(ρA) = 122.5 / (1.225 × 0.5) = 200
        let rate = MassFlowRate::from_kg_per_s(122.5);
        let v = rate.velocity(Density::from_kg_per_m3(1.225), Area::from_m2(0.5));
        assert_relative_eq!(v.as_m_per_sec(), 200.0, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let r = MassFlowRate::default();
        assert!(r.is_zero());
        assert_eq!(r.default_unit_value(), 0.0);
        assert_eq!(r.unit_display_name(), "MassFlowRate (kg/s)");

        let mut r2 = MassFlowRate::default();
        r2.set_value(122.5);
        assert_relative_eq!(r2.as_kg_per_s(), 122.5, epsilon = 1e-12);
        assert!(!r2.is_zero());

        assert!(r2.as_any().downcast_ref::<MassFlowRate>().is_some());
    }
}
