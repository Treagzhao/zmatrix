use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, Distance, HeatFlux, PhysicalQuantity, TemperatureDifference, ThermalConductivity,
    ThermalConductivityType,
};

/// `1 mW/(cm·K)` 等于多少 `W/(m·K)`。
///
/// 与热流密度的 `mW/cm²` 方向相反，容易记混，故单独命名：
/// `1 mW / (1 cm · 1 K) = 1e-3 W / (1e-2 m · K) = 0.1 W/(m·K)`
///
/// 对比：`1 mW/cm² = 10 W/m²`（分子分母都差一个 cm）。
const MW_PER_CM_K_TO_W_PER_M_K: f64 = 0.1;

impl Default for ThermalConductivity {
    fn default() -> Self {
        Self::from_w_per_m_k(0.0)
    }
}

impl PhysicalQuantity for ThermalConductivity {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_w_per_m_k()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "ThermalConductivity (W/(m·K))".to_string()
    }
}

impl ThermalConductivity {
    pub fn from_w_per_m_k(v: f64) -> Self {
        Self {
            default_type: ThermalConductivityType::WattPerMeterKelvin,
            v,
        }
    }

    pub fn from_kw_per_m_k(v: f64) -> Self {
        Self {
            default_type: ThermalConductivityType::KiloWattPerMeterKelvin,
            v,
        }
    }

    pub fn from_mw_per_cm_k(v: f64) -> Self {
        Self {
            default_type: ThermalConductivityType::MilliWattPerCentiMeterKelvin,
            v,
        }
    }

    pub fn as_w_per_m_k(&self) -> f64 {
        match self.default_type {
            ThermalConductivityType::WattPerMeterKelvin => self.v,
            ThermalConductivityType::KiloWattPerMeterKelvin => self.v * 1e3,
            ThermalConductivityType::MilliWattPerCentiMeterKelvin => {
                self.v * MW_PER_CM_K_TO_W_PER_M_K
            }
        }
    }

    pub fn as_kw_per_m_k(&self) -> f64 {
        match self.default_type {
            ThermalConductivityType::WattPerMeterKelvin => self.v * 1e-3,
            ThermalConductivityType::KiloWattPerMeterKelvin => self.v,
            ThermalConductivityType::MilliWattPerCentiMeterKelvin => {
                self.v * MW_PER_CM_K_TO_W_PER_M_K * 1e-3
            }
        }
    }

    pub fn as_mw_per_cm_k(&self) -> f64 {
        match self.default_type {
            ThermalConductivityType::WattPerMeterKelvin => self.v / MW_PER_CM_K_TO_W_PER_M_K,
            ThermalConductivityType::KiloWattPerMeterKelvin => {
                self.v * 1e3 / MW_PER_CM_K_TO_W_PER_M_K
            }
            ThermalConductivityType::MilliWattPerCentiMeterKelvin => self.v,
        }
    }

    /// 傅里叶定律的一维形式：`q = k · ΔT / L`。
    ///
    /// - `delta_t` 必须是 [`TemperatureDifference`] 而非 [`Temperature`](crate::physics::basic::Temperature)：
    ///   传绝对温度会得到毫无意义的巨大热流，而类型系统在这里正好能挡住。
    /// - `thickness` 为 0 时结果为无穷大（无厚度的理想导体），不 panic ——
    ///   与浮点语义一致，调用方自行判断物理合理性。
    ///
    /// ```
    /// use zmatrix::physics::basic::{Distance, TemperatureDifference, ThermalConductivity};
    ///
    /// let k = ThermalConductivity::from_w_per_m_k(237.0); // 铝
    /// let q = k.heat_flux(
    ///     TemperatureDifference::from_celsius(20.0),
    ///     Distance::from_m(0.01),
    /// );
    /// assert!((q.as_w_per_m2() - 474_000.0).abs() < 1e-6);
    /// ```
    pub fn heat_flux(&self, delta_t: TemperatureDifference, thickness: Distance) -> HeatFlux {
        HeatFlux::from_w_per_m2(self.as_w_per_m_k() * delta_t.as_kelvin() / thickness.as_m())
    }

    /// 由实测热流密度、温差、厚度反推导热系数。
    ///
    /// 与 [`heat_flux`](Self::heat_flux) 互逆，用于从实验数据标定材料参数。
    ///
    /// ```
    /// use zmatrix::physics::basic::{
    ///     Distance, HeatFlux, TemperatureDifference, ThermalConductivity,
    /// };
    ///
    /// let k = ThermalConductivity::from_flux(
    ///     HeatFlux::from_w_per_m2(474_000.0),
    ///     TemperatureDifference::from_celsius(20.0),
    ///     Distance::from_m(0.01),
    /// );
    /// assert!((k.as_w_per_m_k() - 237.0).abs() < 1e-9);
    /// ```
    pub fn from_flux(
        flux: HeatFlux,
        delta_t: TemperatureDifference,
        thickness: Distance,
    ) -> Self {
        Self::from_w_per_m_k(
            flux.as_w_per_m2() * thickness.as_m() / delta_t.as_kelvin(),
        )
    }
}

impl Add for ThermalConductivity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() + rhs.as_w_per_m_k())
    }
}

impl<'a, 'b> Add<&'b ThermalConductivity> for &'a ThermalConductivity {
    type Output = ThermalConductivity;
    fn add(self, rhs: &'b ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() + rhs.as_w_per_m_k())
    }
}
impl<'a> Add<&'a ThermalConductivity> for ThermalConductivity {
    type Output = ThermalConductivity;
    fn add(self, rhs: &'a ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() + rhs.as_w_per_m_k())
    }
}
impl<'a> Add<ThermalConductivity> for &'a ThermalConductivity {
    type Output = ThermalConductivity;
    fn add(self, rhs: ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() + rhs.as_w_per_m_k())
    }
}

impl Sub for ThermalConductivity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() - rhs.as_w_per_m_k())
    }
}

impl<'a, 'b> Sub<&'b ThermalConductivity> for &'a ThermalConductivity {
    type Output = ThermalConductivity;
    fn sub(self, rhs: &'b ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() - rhs.as_w_per_m_k())
    }
}
impl<'a> Sub<&'a ThermalConductivity> for ThermalConductivity {
    type Output = ThermalConductivity;
    fn sub(self, rhs: &'a ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() - rhs.as_w_per_m_k())
    }
}
impl<'a> Sub<ThermalConductivity> for &'a ThermalConductivity {
    type Output = ThermalConductivity;
    fn sub(self, rhs: ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self.as_w_per_m_k() - rhs.as_w_per_m_k())
    }
}

impl Neg for ThermalConductivity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_w_per_m_k(-self.as_w_per_m_k())
    }
}

impl Mul<f64> for ThermalConductivity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() * rhs)
    }
}

impl Div<f64> for ThermalConductivity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() / rhs)
    }
}

impl Mul<ThermalConductivity> for f64 {
    type Output = ThermalConductivity;
    fn mul(self, rhs: ThermalConductivity) -> Self::Output {
        ThermalConductivity::from_w_per_m_k(self * rhs.as_w_per_m_k())
    }
}

impl Mul<Coef> for ThermalConductivity {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() * rhs.get_value())
    }
}

impl Div<Coef> for ThermalConductivity {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_m_k(self.as_w_per_m_k() / rhs.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        // 铝
        let k = ThermalConductivity::from_w_per_m_k(237.0);
        assert_relative_eq!(k.as_w_per_m_k(), 237.0, epsilon = 1e-9);
        assert_relative_eq!(k.as_kw_per_m_k(), 0.237, epsilon = 1e-9);
        assert_relative_eq!(k.as_mw_per_cm_k(), 2370.0, epsilon = 1e-9);

        // 隔热毡：0.04 W/(m·K) = 0.4 mW/(cm·K)
        let k = ThermalConductivity::from_mw_per_cm_k(0.4);
        assert_relative_eq!(k.as_w_per_m_k(), 0.04, epsilon = 1e-9);

        // 关键换算：1 mW/(cm·K) = 0.1 W/(m·K)，与热流密度的 ×10 相反
        let k = ThermalConductivity::from_mw_per_cm_k(1.0);
        assert_relative_eq!(k.as_w_per_m_k(), 0.1, epsilon = 1e-9);

        // kW 分支
        let k = ThermalConductivity::from_kw_per_m_k(1.5);
        assert_relative_eq!(k.as_w_per_m_k(), 1500.0, epsilon = 1e-9);
        assert_relative_eq!(k.as_mw_per_cm_k(), 15000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 0.04, 1.0, 237.0, 1e5] {
            let k = ThermalConductivity::from_w_per_m_k(v);
            assert_relative_eq!(
                ThermalConductivity::from_kw_per_m_k(k.as_kw_per_m_k()).as_w_per_m_k(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                ThermalConductivity::from_mw_per_cm_k(k.as_mw_per_cm_k()).as_w_per_m_k(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_unit_independence_in_arith() {
        let a = ThermalConductivity::from_w_per_m_k(100.0);
        let b = ThermalConductivity::from_mw_per_cm_k(1000.0); // = 100 W/(m·K)
        assert_relative_eq!(a.as_w_per_m_k(), b.as_w_per_m_k(), epsilon = 1e-9);

        assert_relative_eq!((a + b).as_w_per_m_k(), 200.0, epsilon = 1e-9);
        assert_relative_eq!((a - b).as_w_per_m_k(), 0.0, epsilon = 1e-9);

        // 混合单位求和：铝 + 钛
        let alu = ThermalConductivity::from_w_per_m_k(237.0);
        let ti = ThermalConductivity::from_mw_per_cm_k(220.0); // = 22 W/(m·K)
        assert_relative_eq!((alu + ti).as_w_per_m_k(), 259.0, epsilon = 1e-9);
    }

    #[test]
    fn test_scalar_arith() {
        let k = ThermalConductivity::from_w_per_m_k(100.0);
        assert_relative_eq!((k * 2.0).as_w_per_m_k(), 200.0, epsilon = 1e-9);
        assert_relative_eq!((k / 4.0).as_w_per_m_k(), 25.0, epsilon = 1e-9);
        assert_relative_eq!((0.5 * k).as_w_per_m_k(), 50.0, epsilon = 1e-9);
        assert_relative_eq!((k * Coef::new(3.0)).as_w_per_m_k(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((k / Coef::new(2.0)).as_w_per_m_k(), 50.0, epsilon = 1e-9);
        assert_relative_eq!((-k).as_w_per_m_k(), -100.0, epsilon = 1e-9);
    }

    #[test]
    fn test_fourier_law() {
        // 铝板 10 mm，温差 20 K：q = 237 × 20 / 0.01 = 474000 W/m²
        let k = ThermalConductivity::from_w_per_m_k(237.0);
        let q = k.heat_flux(
            TemperatureDifference::from_celsius(20.0),
            Distance::from_m(0.01),
        );
        assert_relative_eq!(q.as_w_per_m2(), 474_000.0, epsilon = 1e-6);

        // 厚度加倍 → 热流减半
        let q2 = k.heat_flux(
            TemperatureDifference::from_celsius(20.0),
            Distance::from_m(0.02),
        );
        assert_relative_eq!(q2.as_w_per_m2(), 237_000.0, epsilon = 1e-6);

        // 温差加倍 → 热流加倍
        let q3 = k.heat_flux(
            TemperatureDifference::from_celsius(40.0),
            Distance::from_m(0.01),
        );
        assert_relative_eq!(q3.as_w_per_m2(), 948_000.0, epsilon = 1e-6);

        // 隔热毡同厚度同温差：k 小 6000 倍以上
        let insulation = ThermalConductivity::from_mw_per_cm_k(0.4); // 0.04 W/(m·K)
        let q4 = insulation.heat_flux(
            TemperatureDifference::from_celsius(20.0),
            Distance::from_m(0.01),
        );
        assert_relative_eq!(q4.as_w_per_m2(), 80.0, epsilon = 1e-9);
        assert_relative_eq!(q.as_w_per_m2() / q4.as_w_per_m2(), 5925.0, epsilon = 1e-9);
    }

    #[test]
    fn test_fourier_law_zero_thickness() {
        // 零厚度：热流发散为无穷大，不 panic（浮点语义）
        let k = ThermalConductivity::from_w_per_m_k(237.0);
        let q = k.heat_flux(
            TemperatureDifference::from_celsius(20.0),
            Distance::from_m(0.0),
        );
        assert!(q.as_w_per_m2().is_infinite());
    }

    #[test]
    fn test_from_flux_is_inverse() {
        let k = ThermalConductivity::from_w_per_m_k(237.0);
        let dt = TemperatureDifference::from_celsius(20.0);
        let l = Distance::from_m(0.01);

        let q = k.heat_flux(dt, l);
        let k_back = ThermalConductivity::from_flux(q, dt, l);
        assert_relative_eq!(k_back.as_w_per_m_k(), 237.0, epsilon = 1e-9);

        // 不同单位构造的输入也应得到同一结果
        let q_mw = HeatFlux::from_w_per_m2(q.as_w_per_m2());
        let dt_f = TemperatureDifference::from_fahrenheit(36.0); // = 20 K
        let l_mm = Distance::from_m(0.01);
        let k2 = ThermalConductivity::from_flux(q_mw, dt_f, l_mm);
        assert_relative_eq!(k2.as_w_per_m_k(), 237.0, epsilon = 1e-6);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let k = ThermalConductivity::default();
        assert_relative_eq!(k.as_w_per_m_k(), 0.0, epsilon = 1e-12);
        assert!(k.is_zero());
        assert_eq!(k.default_unit_value(), 0.0);
        assert_eq!(k.unit_display_name(), "ThermalConductivity (W/(m·K))");

        let mut k2 = ThermalConductivity::default();
        k2.set_value(50.0);
        assert_relative_eq!(k2.as_w_per_m_k(), 50.0, epsilon = 1e-12);
        assert!(!k2.is_zero());

        let any = k2.as_any();
        assert!(any.downcast_ref::<ThermalConductivity>().is_some());
    }

    #[test]
    fn test_all_unit_branches() {
        // 三种构造方式表示同一个 100 W/(m·K)，所有取值方法结果必须一致
        let variants = [
            ThermalConductivity::from_w_per_m_k(100.0),
            ThermalConductivity::from_kw_per_m_k(0.1),
            ThermalConductivity::from_mw_per_cm_k(1000.0),
        ];
        for k in variants {
            assert_relative_eq!(k.as_w_per_m_k(), 100.0, epsilon = 1e-9);
            assert_relative_eq!(k.as_kw_per_m_k(), 0.1, epsilon = 1e-9);
            assert_relative_eq!(k.as_mw_per_cm_k(), 1000.0, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_ref_variants() {
        let a = ThermalConductivity::from_w_per_m_k(100.0);
        let b = ThermalConductivity::from_w_per_m_k(40.0);

        assert_relative_eq!((a + b).as_w_per_m_k(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_w_per_m_k(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_w_per_m_k(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_w_per_m_k(), 140.0, epsilon = 1e-9);

        assert_relative_eq!((a - b).as_w_per_m_k(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_w_per_m_k(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_w_per_m_k(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_w_per_m_k(), 60.0, epsilon = 1e-9);
    }
}
