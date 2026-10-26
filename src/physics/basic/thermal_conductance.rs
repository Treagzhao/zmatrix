use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, PhysicalQuantity, Power, TemperatureDifference, ThermalConductance,
    ThermalConductanceType,
};

/// `1 mW/K` 等于多少 `W/K`
const MILL_W_PER_K_TO_W_PER_K: f64 = 1e-3;

impl Default for ThermalConductance {
    fn default() -> Self {
        Self::from_w_per_k(0.0)
    }
}

impl PhysicalQuantity for ThermalConductance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_w_per_k()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "ThermalConductance (W/K)".to_string()
    }
}

impl ThermalConductance {
    pub fn from_w_per_k(v: f64) -> Self {
        Self {
            default_type: ThermalConductanceType::WattPerKelvin,
            v,
        }
    }

    pub fn from_mill_w_per_k(v: f64) -> Self {
        Self {
            default_type: ThermalConductanceType::MilliWattPerKelvin,
            v,
        }
    }

    pub fn as_w_per_k(&self) -> f64 {
        match self.default_type {
            ThermalConductanceType::WattPerKelvin => self.v,
            ThermalConductanceType::MilliWattPerKelvin => self.v * MILL_W_PER_K_TO_W_PER_K,
        }
    }

    pub fn as_mill_w_per_k(&self) -> f64 {
        match self.default_type {
            ThermalConductanceType::WattPerKelvin => self.v / MILL_W_PER_K_TO_W_PER_K,
            ThermalConductanceType::MilliWattPerKelvin => self.v,
        }
    }

    /// 热流：`Q̇ = G · ΔT`。
    ///
    /// 参数是 [`TemperatureDifference`] 而非 [`Temperature`](crate::physics::basic::Temperature)：
    /// 决定传热的是两端**温差**，不是某一端的绝对温度。
    pub fn heat_flow(&self, delta_t: TemperatureDifference) -> Power {
        Power::from_watt(self.as_w_per_k() * delta_t.as_kelvin())
    }

    /// 由热流与温差反推热导：`G = Q̇ / ΔT`。
    ///
    /// 与 [`heat_flow`](Self::heat_flow) 互逆，用于从实测温度与加热功率
    /// 反算节点之间的等效热导（参数辨识的常见步骤）。
    ///
    /// `delta_t` 为零时返回无穷大，不 panic。
    pub fn from_heat_flow(power: Power, delta_t: TemperatureDifference) -> Self {
        Self::from_w_per_k(power.as_watt() / delta_t.as_kelvin())
    }
}

impl Add for ThermalConductance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() + rhs.as_w_per_k())
    }
}

impl<'a, 'b> Add<&'b ThermalConductance> for &'a ThermalConductance {
    type Output = ThermalConductance;
    fn add(self, rhs: &'b ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() + rhs.as_w_per_k())
    }
}
impl<'a> Add<&'a ThermalConductance> for ThermalConductance {
    type Output = ThermalConductance;
    fn add(self, rhs: &'a ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() + rhs.as_w_per_k())
    }
}
impl<'a> Add<ThermalConductance> for &'a ThermalConductance {
    type Output = ThermalConductance;
    fn add(self, rhs: ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() + rhs.as_w_per_k())
    }
}

impl Sub for ThermalConductance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() - rhs.as_w_per_k())
    }
}

impl<'a, 'b> Sub<&'b ThermalConductance> for &'a ThermalConductance {
    type Output = ThermalConductance;
    fn sub(self, rhs: &'b ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() - rhs.as_w_per_k())
    }
}
impl<'a> Sub<&'a ThermalConductance> for ThermalConductance {
    type Output = ThermalConductance;
    fn sub(self, rhs: &'a ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() - rhs.as_w_per_k())
    }
}
impl<'a> Sub<ThermalConductance> for &'a ThermalConductance {
    type Output = ThermalConductance;
    fn sub(self, rhs: ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self.as_w_per_k() - rhs.as_w_per_k())
    }
}

impl Neg for ThermalConductance {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_w_per_k(-self.as_w_per_k())
    }
}

impl Mul<f64> for ThermalConductance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() * rhs)
    }
}

impl Div<f64> for ThermalConductance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() / rhs)
    }
}

impl Mul<ThermalConductance> for f64 {
    type Output = ThermalConductance;
    fn mul(self, rhs: ThermalConductance) -> Self::Output {
        ThermalConductance::from_w_per_k(self * rhs.as_w_per_k())
    }
}

impl Mul<Coef> for ThermalConductance {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() * rhs.get_value())
    }
}

impl Div<Coef> for ThermalConductance {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_k(self.as_w_per_k() / rhs.get_value())
    }
}

// 热导 × 温差 = 热功率：G · ΔT = Q̇
impl Mul<TemperatureDifference> for ThermalConductance {
    type Output = Power;
    fn mul(self, rhs: TemperatureDifference) -> Self::Output {
        Power::from_watt(self.as_w_per_k() * rhs.as_kelvin())
    }
}

impl Mul<TemperatureDifference> for &ThermalConductance {
    type Output = Power;
    fn mul(self, rhs: TemperatureDifference) -> Self::Output {
        Power::from_watt(self.as_w_per_k() * rhs.as_kelvin())
    }
}

impl<'a> Mul<&'a TemperatureDifference> for ThermalConductance {
    type Output = Power;
    fn mul(self, rhs: &'a TemperatureDifference) -> Self::Output {
        Power::from_watt(self.as_w_per_k() * rhs.as_kelvin())
    }
}

impl<'a, 'b> Mul<&'b TemperatureDifference> for &'a ThermalConductance {
    type Output = Power;
    fn mul(self, rhs: &'b TemperatureDifference) -> Self::Output {
        Power::from_watt(self.as_w_per_k() * rhs.as_kelvin())
    }
}

// 温差 × 热导 = 热功率（交换律）
impl Mul<ThermalConductance> for TemperatureDifference {
    type Output = Power;
    fn mul(self, rhs: ThermalConductance) -> Self::Output {
        Power::from_watt(self.as_kelvin() * rhs.as_w_per_k())
    }
}

impl<'a> Mul<ThermalConductance> for &'a TemperatureDifference {
    type Output = Power;
    fn mul(self, rhs: ThermalConductance) -> Self::Output {
        Power::from_watt(self.as_kelvin() * rhs.as_w_per_k())
    }
}

impl<'a> Mul<&'a ThermalConductance> for TemperatureDifference {
    type Output = Power;
    fn mul(self, rhs: &'a ThermalConductance) -> Self::Output {
        Power::from_watt(self.as_kelvin() * rhs.as_w_per_k())
    }
}

impl<'a, 'b> Mul<&'b ThermalConductance> for &'a TemperatureDifference {
    type Output = Power;
    fn mul(self, rhs: &'b ThermalConductance) -> Self::Output {
        Power::from_watt(self.as_kelvin() * rhs.as_w_per_k())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let g = ThermalConductance::from_w_per_k(12.0);
        assert_relative_eq!(g.as_w_per_k(), 12.0, epsilon = 1e-12);
        assert_relative_eq!(g.as_mill_w_per_k(), 12000.0, epsilon = 1e-9);

        let g = ThermalConductance::from_mill_w_per_k(500.0);
        assert_relative_eq!(g.as_w_per_k(), 0.5, epsilon = 1e-12);
        assert_relative_eq!(g.as_mill_w_per_k(), 500.0, epsilon = 1e-9);
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 0.5, 12.0, 1e4] {
            let g = ThermalConductance::from_w_per_k(v);
            assert_relative_eq!(
                ThermalConductance::from_mill_w_per_k(g.as_mill_w_per_k()).as_w_per_k(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_unit_independence_in_arith() {
        let a = ThermalConductance::from_w_per_k(1.0);
        let b = ThermalConductance::from_mill_w_per_k(1000.0); // = 1 W/K
        assert_relative_eq!(a.as_w_per_k(), b.as_w_per_k(), epsilon = 1e-12);
        assert_relative_eq!((a + b).as_w_per_k(), 2.0, epsilon = 1e-12);
        assert_relative_eq!((a - b).as_w_per_k(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn test_scalar_arith() {
        let g = ThermalConductance::from_w_per_k(10.0);
        assert_relative_eq!((g * 2.0).as_w_per_k(), 20.0, epsilon = 1e-12);
        assert_relative_eq!((g / 4.0).as_w_per_k(), 2.5, epsilon = 1e-12);
        assert_relative_eq!((0.5 * g).as_w_per_k(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((g * Coef::new(3.0)).as_w_per_k(), 30.0, epsilon = 1e-12);
        assert_relative_eq!((g / Coef::new(2.0)).as_w_per_k(), 5.0, epsilon = 1e-12);
        assert_relative_eq!((-g).as_w_per_k(), -10.0, epsilon = 1e-12);
    }

    #[test]
    fn test_ref_variants() {
        let a = ThermalConductance::from_w_per_k(10.0);
        let b = ThermalConductance::from_w_per_k(4.0);

        assert_relative_eq!((a + b).as_w_per_k(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + &b).as_w_per_k(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((a + &b).as_w_per_k(), 14.0, epsilon = 1e-12);
        assert_relative_eq!((&a + b).as_w_per_k(), 14.0, epsilon = 1e-12);

        assert_relative_eq!((a - b).as_w_per_k(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - &b).as_w_per_k(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((a - &b).as_w_per_k(), 6.0, epsilon = 1e-12);
        assert_relative_eq!((&a - b).as_w_per_k(), 6.0, epsilon = 1e-12);
    }

    #[test]
    fn test_heat_flow_formula() {
        // G = 12 W/K，ΔT = 20 K → 240 W
        let g = ThermalConductance::from_w_per_k(12.0);
        let q = g * TemperatureDifference::from_celsius(20.0);
        assert_relative_eq!(q.as_watt(), 240.0, epsilon = 1e-9);

        // 方法形式
        assert_relative_eq!(
            g.heat_flow(TemperatureDifference::from_celsius(20.0)).as_watt(),
            240.0,
            epsilon = 1e-9
        );

        // 温差加倍 → 热流加倍
        assert_relative_eq!(
            (g * TemperatureDifference::from_celsius(40.0)).as_watt(),
            480.0,
            epsilon = 1e-9
        );

        // 负温差 → 反向热流
        assert_relative_eq!(
            (g * TemperatureDifference::from_celsius(-20.0)).as_watt(),
            -240.0,
            epsilon = 1e-9
        );

        // 交换律与引用变体
        let dt = TemperatureDifference::from_celsius(20.0);
        assert_relative_eq!((dt * g).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((&g * dt).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((g * &dt).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((&g * &dt).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((&dt * g).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((dt * &g).as_watt(), 240.0, epsilon = 1e-9);
        assert_relative_eq!((&dt * &g).as_watt(), 240.0, epsilon = 1e-9);
    }

    #[test]
    fn test_from_heat_flow_is_inverse() {
        let g = ThermalConductance::from_w_per_k(12.0);
        let dt = TemperatureDifference::from_celsius(20.0);
        let q = g * dt;
        let back = ThermalConductance::from_heat_flow(q, dt);
        assert_relative_eq!(back.as_w_per_k(), 12.0, epsilon = 1e-9);

        // 零温差 → 无穷大，不 panic
        let inf = ThermalConductance::from_heat_flow(
            Power::from_watt(100.0),
            TemperatureDifference::from_celsius(0.0),
        );
        assert!(inf.as_w_per_k().is_infinite());
    }

    #[test]
    fn test_series_and_parallel() {
        // 并联：热导直接相加
        let g1 = ThermalConductance::from_w_per_k(10.0);
        let g2 = ThermalConductance::from_w_per_k(5.0);
        let parallel = g1 + g2;
        assert_relative_eq!(parallel.as_w_per_k(), 15.0, epsilon = 1e-12);

        // 串联：热阻相加，即热导的倒数和
        let r1 = 1.0 / g1.as_w_per_k();
        let r2 = 1.0 / g2.as_w_per_k();
        let series = ThermalConductance::from_w_per_k(1.0 / (r1 + r2));
        assert_relative_eq!(series.as_w_per_k(), 1.0 / (0.1 + 0.2), epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let g = ThermalConductance::default();
        assert!(g.is_zero());
        assert_eq!(g.default_unit_value(), 0.0);
        assert_eq!(g.unit_display_name(), "ThermalConductance (W/K)");

        let mut g2 = ThermalConductance::default();
        g2.set_value(7.5);
        assert_relative_eq!(g2.as_w_per_k(), 7.5, epsilon = 1e-12);
        assert!(!g2.is_zero());

        let any = g2.as_any();
        assert!(any.downcast_ref::<ThermalConductance>().is_some());
    }
}
