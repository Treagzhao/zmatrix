use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, Energy, HeatCapacity, HeatCapacityType, PhysicalQuantity, TemperatureDifference,
};

/// `1 kJ/K` 等于多少 `J/K`
const KJ_PER_K_TO_J_PER_K: f64 = 1e3;

impl Default for HeatCapacity {
    fn default() -> Self {
        Self::from_j_per_k(0.0)
    }
}

impl PhysicalQuantity for HeatCapacity {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_j_per_k()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "HeatCapacity (J/K)".to_string()
    }
}

impl HeatCapacity {
    pub fn from_j_per_k(v: f64) -> Self {
        Self {
            default_type: HeatCapacityType::JoulePerKelvin,
            v,
        }
    }

    pub fn from_kj_per_k(v: f64) -> Self {
        Self {
            default_type: HeatCapacityType::KiloJoulePerKelvin,
            v,
        }
    }

    pub fn as_j_per_k(&self) -> f64 {
        match self.default_type {
            HeatCapacityType::JoulePerKelvin => self.v,
            HeatCapacityType::KiloJoulePerKelvin => self.v * KJ_PER_K_TO_J_PER_K,
        }
    }

    pub fn as_kj_per_k(&self) -> f64 {
        match self.default_type {
            HeatCapacityType::JoulePerKelvin => self.v / KJ_PER_K_TO_J_PER_K,
            HeatCapacityType::KiloJoulePerKelvin => self.v,
        }
    }

    /// 储热：`Q = C · ΔT`。
    ///
    /// 参数是 [`TemperatureDifference`] 而非 [`Temperature`](crate::physics::basic::Temperature)：
    /// 热容描述的是温度**变化**吸收的能量，不是绝对温度本身。
    /// 传绝对温度会得到无意义的巨大能量，类型系统在这里正好挡得住。
    pub fn energy(&self, delta_t: TemperatureDifference) -> Energy {
        Energy::from_joule(self.as_j_per_k() * delta_t.as_kelvin())
    }

    /// 由吸收的热量与温升反推热容：`C = Q / ΔT`。
    ///
    /// 与 [`energy`](Self::energy) 互逆，用于从实测数据标定节点热容。
    ///
    /// `delta_t` 为零时返回无穷大（无热容的理想体），不 panic。
    pub fn from_energy(energy: Energy, delta_t: TemperatureDifference) -> Self {
        Self::from_j_per_k(energy.as_joule() / delta_t.as_kelvin())
    }
}

impl Add for HeatCapacity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() + rhs.as_j_per_k())
    }
}

impl<'a, 'b> Add<&'b HeatCapacity> for &'a HeatCapacity {
    type Output = HeatCapacity;
    fn add(self, rhs: &'b HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() + rhs.as_j_per_k())
    }
}
impl<'a> Add<&'a HeatCapacity> for HeatCapacity {
    type Output = HeatCapacity;
    fn add(self, rhs: &'a HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() + rhs.as_j_per_k())
    }
}
impl<'a> Add<HeatCapacity> for &'a HeatCapacity {
    type Output = HeatCapacity;
    fn add(self, rhs: HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() + rhs.as_j_per_k())
    }
}

impl Sub for HeatCapacity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() - rhs.as_j_per_k())
    }
}

impl<'a, 'b> Sub<&'b HeatCapacity> for &'a HeatCapacity {
    type Output = HeatCapacity;
    fn sub(self, rhs: &'b HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() - rhs.as_j_per_k())
    }
}
impl<'a> Sub<&'a HeatCapacity> for HeatCapacity {
    type Output = HeatCapacity;
    fn sub(self, rhs: &'a HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() - rhs.as_j_per_k())
    }
}
impl<'a> Sub<HeatCapacity> for &'a HeatCapacity {
    type Output = HeatCapacity;
    fn sub(self, rhs: HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self.as_j_per_k() - rhs.as_j_per_k())
    }
}

impl Neg for HeatCapacity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_j_per_k(-self.as_j_per_k())
    }
}

impl Mul<f64> for HeatCapacity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() * rhs)
    }
}

impl Div<f64> for HeatCapacity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() / rhs)
    }
}

impl Mul<HeatCapacity> for f64 {
    type Output = HeatCapacity;
    fn mul(self, rhs: HeatCapacity) -> Self::Output {
        HeatCapacity::from_j_per_k(self * rhs.as_j_per_k())
    }
}

impl Mul<Coef> for HeatCapacity {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() * rhs.get_value())
    }
}

impl Div<Coef> for HeatCapacity {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_j_per_k(self.as_j_per_k() / rhs.get_value())
    }
}

// 热容 × 温差 = 能量：C · ΔT = Q
impl Mul<TemperatureDifference> for HeatCapacity {
    type Output = Energy;
    fn mul(self, rhs: TemperatureDifference) -> Self::Output {
        Energy::from_joule(self.as_j_per_k() * rhs.as_kelvin())
    }
}

impl Mul<TemperatureDifference> for &HeatCapacity {
    type Output = Energy;
    fn mul(self, rhs: TemperatureDifference) -> Self::Output {
        Energy::from_joule(self.as_j_per_k() * rhs.as_kelvin())
    }
}

impl<'a> Mul<&'a TemperatureDifference> for HeatCapacity {
    type Output = Energy;
    fn mul(self, rhs: &'a TemperatureDifference) -> Self::Output {
        Energy::from_joule(self.as_j_per_k() * rhs.as_kelvin())
    }
}

impl<'a, 'b> Mul<&'b TemperatureDifference> for &'a HeatCapacity {
    type Output = Energy;
    fn mul(self, rhs: &'b TemperatureDifference) -> Self::Output {
        Energy::from_joule(self.as_j_per_k() * rhs.as_kelvin())
    }
}

// 温差 × 热容 = 能量（交换律）
impl Mul<HeatCapacity> for TemperatureDifference {
    type Output = Energy;
    fn mul(self, rhs: HeatCapacity) -> Self::Output {
        Energy::from_joule(self.as_kelvin() * rhs.as_j_per_k())
    }
}

impl<'a> Mul<HeatCapacity> for &'a TemperatureDifference {
    type Output = Energy;
    fn mul(self, rhs: HeatCapacity) -> Self::Output {
        Energy::from_joule(self.as_kelvin() * rhs.as_j_per_k())
    }
}

impl<'a> Mul<&'a HeatCapacity> for TemperatureDifference {
    type Output = Energy;
    fn mul(self, rhs: &'a HeatCapacity) -> Self::Output {
        Energy::from_joule(self.as_kelvin() * rhs.as_j_per_k())
    }
}

impl<'a, 'b> Mul<&'b HeatCapacity> for &'a TemperatureDifference {
    type Output = Energy;
    fn mul(self, rhs: &'b HeatCapacity) -> Self::Output {
        Energy::from_joule(self.as_kelvin() * rhs.as_j_per_k())
    }
}

// 能量 ÷ 热容 = 温差：Q / C = ΔT
//
// 与 `C · ΔT = Q` 互为逆运算。这是瞬态热网络的主力路径：
// 已知一个节点吸收的能量与自身热容，反解它升温多少。
impl Div<HeatCapacity> for Energy {
    type Output = TemperatureDifference;
    fn div(self, rhs: HeatCapacity) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_joule() / rhs.as_j_per_k())
    }
}

impl Div<HeatCapacity> for &Energy {
    type Output = TemperatureDifference;
    fn div(self, rhs: HeatCapacity) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_joule() / rhs.as_j_per_k())
    }
}

impl<'a> Div<&'a HeatCapacity> for Energy {
    type Output = TemperatureDifference;
    fn div(self, rhs: &'a HeatCapacity) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_joule() / rhs.as_j_per_k())
    }
}

impl<'a, 'b> Div<&'b HeatCapacity> for &'a Energy {
    type Output = TemperatureDifference;
    fn div(self, rhs: &'b HeatCapacity) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_joule() / rhs.as_j_per_k())
    }
}

// 热容 ÷ 质量 = 比热容（比热容类型尚未实现，故不提供此运算）

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let c = HeatCapacity::from_j_per_k(9000.0);
        assert_relative_eq!(c.as_j_per_k(), 9000.0, epsilon = 1e-9);
        assert_relative_eq!(c.as_kj_per_k(), 9.0, epsilon = 1e-9);

        let c = HeatCapacity::from_kj_per_k(9.0);
        assert_relative_eq!(c.as_j_per_k(), 9000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 1.0, 450.0, 9000.0, 1e6] {
            let c = HeatCapacity::from_j_per_k(v);
            assert_relative_eq!(
                HeatCapacity::from_kj_per_k(c.as_kj_per_k()).as_j_per_k(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_unit_independence_in_arith() {
        let a = HeatCapacity::from_j_per_k(1000.0);
        let b = HeatCapacity::from_kj_per_k(1.0); // = 1000 J/K
        assert_relative_eq!(a.as_j_per_k(), b.as_j_per_k(), epsilon = 1e-9);

        assert_relative_eq!((a + b).as_j_per_k(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((a - b).as_j_per_k(), 0.0, epsilon = 1e-9);
    }

    #[test]
    fn test_scalar_arith() {
        let c = HeatCapacity::from_j_per_k(1000.0);
        assert_relative_eq!((c * 2.0).as_j_per_k(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((c / 4.0).as_j_per_k(), 250.0, epsilon = 1e-9);
        assert_relative_eq!((0.5 * c).as_j_per_k(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((c * Coef::new(3.0)).as_j_per_k(), 3000.0, epsilon = 1e-9);
        assert_relative_eq!((c / Coef::new(2.0)).as_j_per_k(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((-c).as_j_per_k(), -1000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_ref_variants() {
        let a = HeatCapacity::from_j_per_k(1000.0);
        let b = HeatCapacity::from_j_per_k(400.0);

        assert_relative_eq!((a + b).as_j_per_k(), 1400.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_j_per_k(), 1400.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_j_per_k(), 1400.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_j_per_k(), 1400.0, epsilon = 1e-9);

        assert_relative_eq!((a - b).as_j_per_k(), 600.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_j_per_k(), 600.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_j_per_k(), 600.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_j_per_k(), 600.0, epsilon = 1e-9);
    }

    #[test]
    fn test_energy_formula() {
        // 10 kg 铝：C = 900 × 10 = 9000 J/K，升温 20 K 需 180 kJ
        let c = HeatCapacity::from_j_per_k(9000.0);
        let q = c * TemperatureDifference::from_celsius(20.0);
        assert_relative_eq!(q.as_joule(), 180_000.0, epsilon = 1e-6);

        // 方法形式与运算符等价
        assert_relative_eq!(
            c.energy(TemperatureDifference::from_celsius(20.0)).as_joule(),
            180_000.0,
            epsilon = 1e-6
        );

        // 温升加倍 → 能量加倍
        let q2 = c * TemperatureDifference::from_celsius(40.0);
        assert_relative_eq!(q2.as_joule(), 360_000.0, epsilon = 1e-6);

        // 负温差（降温）→ 负能量（放热）
        let q3 = c * TemperatureDifference::from_celsius(-20.0);
        assert_relative_eq!(q3.as_joule(), -180_000.0, epsilon = 1e-6);

        // 交换律
        let dt = TemperatureDifference::from_celsius(20.0);
        assert_relative_eq!((dt * c).as_joule(), 180_000.0, epsilon = 1e-6);

        // 引用变体
        assert_relative_eq!((&c * dt).as_joule(), 180_000.0, epsilon = 1e-6);
        assert_relative_eq!((c * &dt).as_joule(), 180_000.0, epsilon = 1e-6);
        assert_relative_eq!((&c * &dt).as_joule(), 180_000.0, epsilon = 1e-6);
        assert_relative_eq!((&dt * c).as_joule(), 180_000.0, epsilon = 1e-6);
        assert_relative_eq!((dt * &c).as_joule(), 180_000.0, epsilon = 1e-6);
        assert_relative_eq!((&dt * &c).as_joule(), 180_000.0, epsilon = 1e-6);
    }

    #[test]
    fn test_energy_with_fahrenheit_difference() {
        // 20 K 温升 = 36 °F 温升，能量必须相同
        let c = HeatCapacity::from_j_per_k(9000.0);
        let q_k = c * TemperatureDifference::from_celsius(20.0);
        let q_f = c * TemperatureDifference::from_fahrenheit(36.0);
        assert_relative_eq!(q_k.as_joule(), q_f.as_joule(), epsilon = 1e-6);
    }

    #[test]
    fn test_from_energy_is_inverse() {
        let c = HeatCapacity::from_j_per_k(9000.0);
        let dt = TemperatureDifference::from_celsius(20.0);
        let q = c * dt;
        let back = HeatCapacity::from_energy(q, dt);
        assert_relative_eq!(back.as_j_per_k(), 9000.0, epsilon = 1e-6);

        // 零温差 → 无穷大热容，不 panic
        let inf = HeatCapacity::from_energy(
            Energy::from_joule(100.0),
            TemperatureDifference::from_celsius(0.0),
        );
        assert!(inf.as_j_per_k().is_infinite());
    }

    #[test]
    fn test_energy_div_capacity_gives_difference() {
        // Q / C = ΔT：180 kJ 加进 9000 J/K 的节点，升温 20 K
        let c = HeatCapacity::from_j_per_k(9000.0);
        let q = Energy::from_joule(180_000.0);
        let dt = q / c;
        assert_relative_eq!(dt.as_kelvin(), 20.0, epsilon = 1e-9);

        // 4 个引用变体
        assert_relative_eq!((&q / c).as_kelvin(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((q / &c).as_kelvin(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((&q / &c).as_kelvin(), 20.0, epsilon = 1e-9);

        // 与 C·ΔT 互为逆运算
        assert_relative_eq!((c * dt).as_joule(), q.as_joule(), epsilon = 1e-6);

        // 负能量（放热）→ 负温差
        let cool = Energy::from_joule(-90_000.0) / c;
        assert_relative_eq!(cool.as_kelvin(), -10.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 两种构造方式表示同一个 9000 J/K，所有取值方法结果必须一致
        for c in [
            HeatCapacity::from_j_per_k(9000.0),
            HeatCapacity::from_kj_per_k(9.0),
        ] {
            assert_relative_eq!(c.as_j_per_k(), 9000.0, epsilon = 1e-9);
            assert_relative_eq!(c.as_kj_per_k(), 9.0, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_physical_quantity_contract() {
        let c = HeatCapacity::default();
        assert!(c.is_zero());
        assert_eq!(c.default_unit_value(), 0.0);
        assert_eq!(c.unit_display_name(), "HeatCapacity (J/K)");

        let mut c2 = HeatCapacity::default();
        c2.set_value(500.0);
        assert_relative_eq!(c2.as_j_per_k(), 500.0, epsilon = 1e-12);
        assert!(!c2.is_zero());

        let any = c2.as_any();
        assert!(any.downcast_ref::<HeatCapacity>().is_some());
    }
}
