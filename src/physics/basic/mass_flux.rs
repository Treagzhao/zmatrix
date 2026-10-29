use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Coef, Density, MassFlux, MassFluxType, PhysicalQuantity, Pressure, Velocity,
};

/// `1 g/(m²·s)` 等于多少 `kg/(m²·s)`。
const G_PER_M2_S_TO_KG: f64 = 1e-3;

/// `1 g/(cm²·s)` 等于多少 `kg/(m²·s)`。
///
/// 10，不是 1e-3 —— 分母是 cm² 而非法 m²：
/// `1 g/(cm²·s) = 1e-3 kg / 1e-4 m² / s = 10 kg/(m²·s)`。
///
/// 与上一层那个 1e-3 相差 10000 倍，和 `HeatFlux` 里 `mW/cm²` 是同一个坑。
const G_PER_CM2_S_TO_KG: f64 = 10.0;

impl Default for MassFlux {
    fn default() -> Self {
        Self::from_kg_per_m2_s(0.0)
    }
}

impl PhysicalQuantity for MassFlux {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_per_m2_s()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "MassFlux (kg/(m²·s))".to_string()
    }
}

impl MassFlux {
    pub fn from_kg_per_m2_s(v: f64) -> Self {
        Self {
            default_type: MassFluxType::KilogramPerSquareMeterSecond,
            v,
        }
    }

    pub fn from_g_per_m2_s(v: f64) -> Self {
        Self {
            default_type: MassFluxType::GramPerSquareMeterSecond,
            v,
        }
    }

    pub fn from_g_per_cm2_s(v: f64) -> Self {
        Self {
            default_type: MassFluxType::GramPerSquareCentiMeterSecond,
            v,
        }
    }

    pub fn as_kg_per_m2_s(&self) -> f64 {
        match self.default_type {
            MassFluxType::KilogramPerSquareMeterSecond => self.v,
            MassFluxType::GramPerSquareMeterSecond => self.v * G_PER_M2_S_TO_KG,
            MassFluxType::GramPerSquareCentiMeterSecond => self.v * G_PER_CM2_S_TO_KG,
        }
    }

    pub fn as_g_per_m2_s(&self) -> f64 {
        match self.default_type {
            MassFluxType::KilogramPerSquareMeterSecond => self.v / G_PER_M2_S_TO_KG,
            MassFluxType::GramPerSquareMeterSecond => self.v,
            MassFluxType::GramPerSquareCentiMeterSecond => {
                self.v * G_PER_CM2_S_TO_KG / G_PER_M2_S_TO_KG
            }
        }
    }

    pub fn as_g_per_cm2_s(&self) -> f64 {
        match self.default_type {
            MassFluxType::KilogramPerSquareMeterSecond => self.v / G_PER_CM2_S_TO_KG,
            MassFluxType::GramPerSquareMeterSecond => {
                self.v * G_PER_M2_S_TO_KG / G_PER_CM2_S_TO_KG
            }
            MassFluxType::GramPerSquareCentiMeterSecond => self.v,
        }
    }

    /// 动压：`q = ρv · v = ρv²`。
    ///
    /// 这是质量通量存在的理由——把 `ρv²` 拆成两步，每步都有物理名。
    /// 完整动压公式是 `q = ½ρv²`，**本方法不含 ½**，系数请自行处理；
    /// 需要带 ½ 的版本用 [`Density::dynamic_pressure`](crate::physics::basic::Density::dynamic_pressure)。
    pub fn dynamic_pressure(&self, velocity: Velocity) -> Pressure {
        Pressure::from_pa(self.as_kg_per_m2_s() * velocity.as_m_per_sec())
    }
}

impl Add for MassFlux {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() + rhs.as_kg_per_m2_s())
    }
}

impl<'a, 'b> Add<&'b MassFlux> for &'a MassFlux {
    type Output = MassFlux;
    fn add(self, rhs: &'b MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() + rhs.as_kg_per_m2_s())
    }
}
impl<'a> Add<&'a MassFlux> for MassFlux {
    type Output = MassFlux;
    fn add(self, rhs: &'a MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() + rhs.as_kg_per_m2_s())
    }
}
impl<'a> Add<MassFlux> for &'a MassFlux {
    type Output = MassFlux;
    fn add(self, rhs: MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() + rhs.as_kg_per_m2_s())
    }
}

impl Sub for MassFlux {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() - rhs.as_kg_per_m2_s())
    }
}

impl<'a, 'b> Sub<&'b MassFlux> for &'a MassFlux {
    type Output = MassFlux;
    fn sub(self, rhs: &'b MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() - rhs.as_kg_per_m2_s())
    }
}
impl<'a> Sub<&'a MassFlux> for MassFlux {
    type Output = MassFlux;
    fn sub(self, rhs: &'a MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() - rhs.as_kg_per_m2_s())
    }
}
impl<'a> Sub<MassFlux> for &'a MassFlux {
    type Output = MassFlux;
    fn sub(self, rhs: MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m2_s() - rhs.as_kg_per_m2_s())
    }
}

impl Neg for MassFlux {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_kg_per_m2_s(-self.as_kg_per_m2_s())
    }
}

impl Mul<f64> for MassFlux {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() * rhs)
    }
}

impl Div<f64> for MassFlux {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() / rhs)
    }
}

impl Mul<MassFlux> for f64 {
    type Output = MassFlux;
    fn mul(self, rhs: MassFlux) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self * rhs.as_kg_per_m2_s())
    }
}

impl Mul<Coef> for MassFlux {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() * rhs.get_value())
    }
}

impl Div<Coef> for MassFlux {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_m2_s(self.as_kg_per_m2_s() / rhs.get_value())
    }
}

// 质量通量 × 速度 = 压力：ρv · v = ρv²（动压，不含 1/2）
impl Mul<Velocity> for MassFlux {
    type Output = Pressure;
    fn mul(self, rhs: Velocity) -> Self::Output {
        Pressure::from_pa(self.as_kg_per_m2_s() * rhs.as_m_per_sec())
    }
}

impl Mul<Velocity> for &MassFlux {
    type Output = Pressure;
    fn mul(self, rhs: Velocity) -> Self::Output {
        Pressure::from_pa(self.as_kg_per_m2_s() * rhs.as_m_per_sec())
    }
}

impl<'a> Mul<&'a Velocity> for MassFlux {
    type Output = Pressure;
    fn mul(self, rhs: &'a Velocity) -> Self::Output {
        Pressure::from_pa(self.as_kg_per_m2_s() * rhs.as_m_per_sec())
    }
}

impl<'a, 'b> Mul<&'b Velocity> for &'a MassFlux {
    type Output = Pressure;
    fn mul(self, rhs: &'b Velocity) -> Self::Output {
        Pressure::from_pa(self.as_kg_per_m2_s() * rhs.as_m_per_sec())
    }
}

// 质量通量 ÷ 速度 = 密度：ρ = (ρv) / v
impl Div<Velocity> for MassFlux {
    type Output = Density;
    fn div(self, rhs: Velocity) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m2_s() / rhs.as_m_per_sec())
    }
}

impl Div<Velocity> for &MassFlux {
    type Output = Density;
    fn div(self, rhs: Velocity) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m2_s() / rhs.as_m_per_sec())
    }
}

impl<'a> Div<&'a Velocity> for MassFlux {
    type Output = Density;
    fn div(self, rhs: &'a Velocity) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m2_s() / rhs.as_m_per_sec())
    }
}

impl<'a, 'b> Div<&'b Velocity> for &'a MassFlux {
    type Output = Density;
    fn div(self, rhs: &'b Velocity) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m2_s() / rhs.as_m_per_sec())
    }
}

// 压力 ÷ 速度 = 质量通量：ρv = ρv²/v
impl Div<Velocity> for Pressure {
    type Output = MassFlux;
    fn div(self, rhs: Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_pa() / rhs.as_m_per_sec())
    }
}

impl Div<Velocity> for &Pressure {
    type Output = MassFlux;
    fn div(self, rhs: Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_pa() / rhs.as_m_per_sec())
    }
}

impl<'a> Div<&'a Velocity> for Pressure {
    type Output = MassFlux;
    fn div(self, rhs: &'a Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_pa() / rhs.as_m_per_sec())
    }
}

impl<'a, 'b> Div<&'b Velocity> for &'a Pressure {
    type Output = MassFlux;
    fn div(self, rhs: &'b Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_pa() / rhs.as_m_per_sec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let mf = MassFlux::from_kg_per_m2_s(10.0);
        assert_relative_eq!(mf.as_kg_per_m2_s(), 10.0, epsilon = 1e-12);
        assert_relative_eq!(mf.as_g_per_m2_s(), 10_000.0, epsilon = 1e-6);
        assert_relative_eq!(mf.as_g_per_cm2_s(), 1.0, epsilon = 1e-12);

        assert_relative_eq!(MassFlux::from_g_per_m2_s(1000.0).as_kg_per_m2_s(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(MassFlux::from_g_per_cm2_s(1.0).as_kg_per_m2_s(), 10.0, epsilon = 1e-12);
    }

    #[test]
    fn test_cm2_unit_is_10_not_1e_minus3() {
        // 本类型最易错的一处：g/(cm²·s) 的因子是 10，不是 1e-3
        let cm2 = MassFlux::from_g_per_cm2_s(1.0);
        let m2 = MassFlux::from_g_per_m2_s(1.0);
        // 两者相差 10000 倍
        assert_relative_eq!(cm2.as_kg_per_m2_s() / m2.as_kg_per_m2_s(), 10_000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 三种构造表示的同一通量
        let a = MassFlux::from_kg_per_m2_s(25.0);
        let b = MassFlux::from_g_per_m2_s(25_000.0);
        let c = MassFlux::from_g_per_cm2_s(2.5);

        for mf in [a, b, c] {
            assert_relative_eq!(mf.as_kg_per_m2_s(), 25.0, epsilon = 1e-9);
            assert_relative_eq!(mf.as_g_per_m2_s(), 25_000.0, epsilon = 1e-4);
            assert_relative_eq!(mf.as_g_per_cm2_s(), 2.5, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 1e-6, 0.5, 25.0, 1e4] {
            let mf = MassFlux::from_kg_per_m2_s(v);
            assert_relative_eq!(
                MassFlux::from_g_per_m2_s(mf.as_g_per_m2_s()).as_kg_per_m2_s(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                MassFlux::from_g_per_cm2_s(mf.as_g_per_cm2_s()).as_kg_per_m2_s(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = MassFlux::from_kg_per_m2_s(10.0);
        let b = MassFlux::from_g_per_cm2_s(1.0); // 同为 10 kg/(m²·s)

        assert_relative_eq!((a + b).as_kg_per_m2_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_kg_per_m2_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_kg_per_m2_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_kg_per_m2_s(), 20.0, epsilon = 1e-9);

        assert_relative_eq!((a - b).as_kg_per_m2_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_kg_per_m2_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_kg_per_m2_s(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_kg_per_m2_s(), 0.0, epsilon = 1e-9);

        assert_relative_eq!((a * 2.0).as_kg_per_m2_s(), 20.0, epsilon = 1e-9);
        assert_relative_eq!((a / 4.0).as_kg_per_m2_s(), 2.5, epsilon = 1e-9);
        assert_relative_eq!((0.5 * a).as_kg_per_m2_s(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((a * Coef::new(3.0)).as_kg_per_m2_s(), 30.0, epsilon = 1e-9);
        assert_relative_eq!((a / Coef::new(2.0)).as_kg_per_m2_s(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((-a).as_kg_per_m2_s(), -10.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mul_velocity_gives_pressure() {
        // 海平面 1.225 kg/m³ × 200 m/s = 245 kg/(m²·s)
        // 再 × 200 = 49000 Pa = ρv²
        let mf = MassFlux::from_kg_per_m2_s(245.0);
        let v = Velocity::from_m_per_sec(200.0);

        let q = mf * v;
        assert_relative_eq!(q.as_pa(), 49_000.0, epsilon = 1e-9);

        assert_relative_eq!((&mf * v).as_pa(), 49_000.0, epsilon = 1e-9);
        assert_relative_eq!((mf * &v).as_pa(), 49_000.0, epsilon = 1e-9);
        assert_relative_eq!((&mf * &v).as_pa(), 49_000.0, epsilon = 1e-9);

        assert_relative_eq!(mf.dynamic_pressure(v).as_pa(), 49_000.0, epsilon = 1e-9);

        // 含 1/2 的完整动压：24500 Pa
        assert_relative_eq!((mf * v * Coef::new(0.5)).as_pa(), 24_500.0, epsilon = 1e-9);
    }

    #[test]
    fn test_inverse_operations() {
        let mf = MassFlux::from_kg_per_m2_s(245.0);
        let v = Velocity::from_m_per_sec(200.0);

        // 通量 ÷ 速度 = 密度
        let rho = mf / v;
        assert_relative_eq!(rho.as_kg_per_m3(), 1.225, epsilon = 1e-9);
        assert_relative_eq!((&mf / &v).as_kg_per_m3(), 1.225, epsilon = 1e-9);
        assert_relative_eq!((mf / &v).as_kg_per_m3(), 1.225, epsilon = 1e-9);
        assert_relative_eq!((&mf / v).as_kg_per_m3(), 1.225, epsilon = 1e-9);

        // 压力 ÷ 速度 = 通量
        let p = Pressure::from_pa(49_000.0);
        let back = p / v;
        assert_relative_eq!(back.as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((&p / &v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((p / &v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((&p / v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
    }

    #[test]
    fn test_chain_matches_single_step_formula() {
        // 两步链 (ρ × v) × v 必须与一步 ρv² 完全一致
        use crate::physics::basic::Density;

        let rho = Density::from_kg_per_m3(1.225);
        let v = Velocity::from_m_per_sec(200.0);

        let two_step = (rho * v) * v;
        let one_step = rho.as_kg_per_m3() * v.as_m_per_sec() * v.as_m_per_sec();

        assert_relative_eq!(two_step.as_pa(), one_step, epsilon = 1e-9);
        assert_relative_eq!(two_step.as_pa(), 49_000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let mf = MassFlux::default();
        assert!(mf.is_zero());
        assert_eq!(mf.default_unit_value(), 0.0);
        assert_eq!(mf.unit_display_name(), "MassFlux (kg/(m²·s))");

        let mut mf2 = MassFlux::default();
        mf2.set_value(25.0);
        assert_relative_eq!(mf2.as_kg_per_m2_s(), 25.0, epsilon = 1e-12);
        assert!(!mf2.is_zero());

        assert!(mf2.as_any().downcast_ref::<MassFlux>().is_some());
    }
}
