use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Area, Coef, Density, DensityType, Force, Mass, MassFlux, PhysicalQuantity, Pressure, Velocity,
    Volume,
};

/// `1 g/cm³` 等于多少 `kg/m³`。
///
/// 1000，不是 1e-3 —— cm³ 是 1e-6 m³，1 g / 1 cm³ = 1e-3 kg / 1e-6 m³ = 1e3 kg/m³。
const G_PER_CM3_TO_KG_PER_M3: f64 = 1e3;

/// `1 g/L` 等于多少 `kg/m³`。
///
/// 1，因为 1 L = 1e-3 m³，1 g = 1e-3 kg，两者相除抵消。
/// 与上面那个 1e3 相差 1000 倍，是密度换算里最容易混淆的一处。
const G_PER_L_TO_KG_PER_M3: f64 = 1.0;

impl Default for Density {
    fn default() -> Self {
        Self::from_kg_per_m3(0.0)
    }
}

impl PhysicalQuantity for Density {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_per_m3()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "Density (kg/m³)".to_string()
    }
}

impl Density {
    pub fn from_kg_per_m3(v: f64) -> Self {
        Self {
            default_type: DensityType::KilogramPerCubicMeter,
            v,
        }
    }

    pub fn from_g_per_cm3(v: f64) -> Self {
        Self {
            default_type: DensityType::GramPerCubicCentiMeter,
            v,
        }
    }

    pub fn from_g_per_liter(v: f64) -> Self {
        Self {
            default_type: DensityType::GramPerLiter,
            v,
        }
    }

    pub fn as_kg_per_m3(&self) -> f64 {
        match self.default_type {
            DensityType::KilogramPerCubicMeter => self.v,
            DensityType::GramPerCubicCentiMeter => self.v * G_PER_CM3_TO_KG_PER_M3,
            DensityType::GramPerLiter => self.v * G_PER_L_TO_KG_PER_M3,
        }
    }

    pub fn as_g_per_cm3(&self) -> f64 {
        match self.default_type {
            DensityType::KilogramPerCubicMeter => self.v / G_PER_CM3_TO_KG_PER_M3,
            DensityType::GramPerCubicCentiMeter => self.v,
            DensityType::GramPerLiter => self.v * G_PER_L_TO_KG_PER_M3 / G_PER_CM3_TO_KG_PER_M3,
        }
    }

    pub fn as_g_per_liter(&self) -> f64 {
        match self.default_type {
            DensityType::KilogramPerCubicMeter => self.v / G_PER_L_TO_KG_PER_M3,
            DensityType::GramPerCubicCentiMeter => {
                self.v * G_PER_CM3_TO_KG_PER_M3 / G_PER_L_TO_KG_PER_M3
            }
            DensityType::GramPerLiter => self.v,
        }
    }

    /// 质量：`m = ρ · V`。
    pub fn mass(&self, volume: Volume) -> Mass {
        Mass::from_kg(self.as_kg_per_m3() * volume.as_m3())
    }

    /// 质量通量：`ρ · v`。
    ///
    /// 这是动压链条的第一段，见 [`MassFlux`] 的说明。
    pub fn mass_flux(&self, velocity: Velocity) -> MassFlux {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m3() * velocity.as_m_per_sec())
    }

    /// 动压：`q = ½ · ρ · v²`。
    ///
    /// **含 1/2 系数。** 运算符 `ρ * v * v` 给的是 `ρv²`，不含 1/2——
    /// 两者差一倍，是气动计算里最常见的错。
    ///
    /// 若要用「质量通量 × 速度」两步走，`MassFlux::dynamic_pressure` 也不含 1/2，
    /// 因为它只负责量纲链；带物理系数的入口只有本方法。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{Density, Velocity};
    ///
    /// let q = Density::from_kg_per_m3(1.225)
    ///     .dynamic_pressure(Velocity::from_m_per_sec(200.0));
    /// assert!((q.as_pa() - 24_500.0).abs() < 1e-9);
    /// ```
    pub fn dynamic_pressure(&self, velocity: Velocity) -> Pressure {
        Pressure::from_pa(0.5 * self.as_kg_per_m3() * velocity.as_m_per_sec().powi(2))
    }

    /// 气动阻力：`F = ½ · ρ · v² · C_D · A`。
    ///
    /// 四项系数里的三个（½、C_D、A）都由运算符表达不了，所以整条公式做成一个方法。
    ///
    /// `drag_coefficient` 用 [`Coef`] 传入，是因为阻力系数本身是无量纲的；
    /// 平板迎风约 1.2、流线型约 0.04–0.1、圆柱约 1.0。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{Area, Coef, Density, Velocity};
    ///
    /// // 海平面 200 m/s，C_D = 1.2，迎风面积 2 m²
    /// let f = Density::from_kg_per_m3(1.225).drag_force(
    ///     Velocity::from_m_per_sec(200.0),
    ///     Coef::new(1.2),
    ///     Area::from_m2(2.0),
    /// );
    /// assert!((f.as_newton() - 58_800.0).abs() < 1e-6);
    /// ```
    pub fn drag_force(&self, velocity: Velocity, drag_coefficient: Coef, area: Area) -> Force {
        let q = self.dynamic_pressure(velocity);
        q.force_on(area) * drag_coefficient
    }
}

impl Add for Density {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() + rhs.as_kg_per_m3())
    }
}

impl<'a, 'b> Add<&'b Density> for &'a Density {
    type Output = Density;
    fn add(self, rhs: &'b Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() + rhs.as_kg_per_m3())
    }
}
impl<'a> Add<&'a Density> for Density {
    type Output = Density;
    fn add(self, rhs: &'a Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() + rhs.as_kg_per_m3())
    }
}
impl<'a> Add<Density> for &'a Density {
    type Output = Density;
    fn add(self, rhs: Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() + rhs.as_kg_per_m3())
    }
}

impl Sub for Density {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() - rhs.as_kg_per_m3())
    }
}

impl<'a, 'b> Sub<&'b Density> for &'a Density {
    type Output = Density;
    fn sub(self, rhs: &'b Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() - rhs.as_kg_per_m3())
    }
}
impl<'a> Sub<&'a Density> for Density {
    type Output = Density;
    fn sub(self, rhs: &'a Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() - rhs.as_kg_per_m3())
    }
}
impl<'a> Sub<Density> for &'a Density {
    type Output = Density;
    fn sub(self, rhs: Density) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg_per_m3() - rhs.as_kg_per_m3())
    }
}

impl Neg for Density {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_kg_per_m3(-self.as_kg_per_m3())
    }
}

impl Mul<f64> for Density {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() * rhs)
    }
}

impl Div<f64> for Density {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() / rhs)
    }
}

impl Mul<Density> for f64 {
    type Output = Density;
    fn mul(self, rhs: Density) -> Self::Output {
        Density::from_kg_per_m3(self * rhs.as_kg_per_m3())
    }
}

impl Mul<Coef> for Density {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() * rhs.get_value())
    }
}

impl Div<Coef> for Density {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_kg_per_m3(self.as_kg_per_m3() / rhs.get_value())
    }
}

// 密度 × 体积 = 质量：m = ρV
impl Mul<Volume> for Density {
    type Output = Mass;
    fn mul(self, rhs: Volume) -> Self::Output {
        Mass::from_kg(self.as_kg_per_m3() * rhs.as_m3())
    }
}

impl Mul<Volume> for &Density {
    type Output = Mass;
    fn mul(self, rhs: Volume) -> Self::Output {
        Mass::from_kg(self.as_kg_per_m3() * rhs.as_m3())
    }
}

impl<'a> Mul<&'a Volume> for Density {
    type Output = Mass;
    fn mul(self, rhs: &'a Volume) -> Self::Output {
        Mass::from_kg(self.as_kg_per_m3() * rhs.as_m3())
    }
}

impl<'a, 'b> Mul<&'b Volume> for &'a Density {
    type Output = Mass;
    fn mul(self, rhs: &'b Volume) -> Self::Output {
        Mass::from_kg(self.as_kg_per_m3() * rhs.as_m3())
    }
}

// 体积 × 密度 = 质量（交换律）
impl Mul<Density> for Volume {
    type Output = Mass;
    fn mul(self, rhs: Density) -> Self::Output {
        Mass::from_kg(self.as_m3() * rhs.as_kg_per_m3())
    }
}

impl<'a> Mul<Density> for &'a Volume {
    type Output = Mass;
    fn mul(self, rhs: Density) -> Self::Output {
        Mass::from_kg(self.as_m3() * rhs.as_kg_per_m3())
    }
}

impl<'a> Mul<&'a Density> for Volume {
    type Output = Mass;
    fn mul(self, rhs: &'a Density) -> Self::Output {
        Mass::from_kg(self.as_m3() * rhs.as_kg_per_m3())
    }
}

impl<'a, 'b> Mul<&'b Density> for &'a Volume {
    type Output = Mass;
    fn mul(self, rhs: &'b Density) -> Self::Output {
        Mass::from_kg(self.as_m3() * rhs.as_kg_per_m3())
    }
}

// 密度 × 速度 = 质量通量：ρv
impl Mul<Velocity> for Density {
    type Output = MassFlux;
    fn mul(self, rhs: Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m3() * rhs.as_m_per_sec())
    }
}

impl Mul<Velocity> for &Density {
    type Output = MassFlux;
    fn mul(self, rhs: Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m3() * rhs.as_m_per_sec())
    }
}

impl<'a> Mul<&'a Velocity> for Density {
    type Output = MassFlux;
    fn mul(self, rhs: &'a Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m3() * rhs.as_m_per_sec())
    }
}

impl<'a, 'b> Mul<&'b Velocity> for &'a Density {
    type Output = MassFlux;
    fn mul(self, rhs: &'b Velocity) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_kg_per_m3() * rhs.as_m_per_sec())
    }
}

// 速度 × 密度 = 质量通量（交换律）
//
// 物理上 ρ 与 v 的顺序无所谓，但动量方程通常写 ρv，这里两个方向都提供。
impl Mul<Density> for Velocity {
    type Output = MassFlux;
    fn mul(self, rhs: Density) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_m_per_sec() * rhs.as_kg_per_m3())
    }
}

impl<'a> Mul<Density> for &'a Velocity {
    type Output = MassFlux;
    fn mul(self, rhs: Density) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_m_per_sec() * rhs.as_kg_per_m3())
    }
}

impl<'a> Mul<&'a Density> for Velocity {
    type Output = MassFlux;
    fn mul(self, rhs: &'a Density) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_m_per_sec() * rhs.as_kg_per_m3())
    }
}

impl<'a, 'b> Mul<&'b Density> for &'a Velocity {
    type Output = MassFlux;
    fn mul(self, rhs: &'b Density) -> Self::Output {
        MassFlux::from_kg_per_m2_s(self.as_m_per_sec() * rhs.as_kg_per_m3())
    }
}

// 质量 ÷ 体积 = 密度：ρ = m/V
impl Div<Volume> for Mass {
    type Output = Density;
    fn div(self, rhs: Volume) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg() / rhs.as_m3())
    }
}

impl Div<Volume> for &Mass {
    type Output = Density;
    fn div(self, rhs: Volume) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg() / rhs.as_m3())
    }
}

impl<'a> Div<&'a Volume> for Mass {
    type Output = Density;
    fn div(self, rhs: &'a Volume) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg() / rhs.as_m3())
    }
}

impl<'a, 'b> Div<&'b Volume> for &'a Mass {
    type Output = Density;
    fn div(self, rhs: &'b Volume) -> Self::Output {
        Density::from_kg_per_m3(self.as_kg() / rhs.as_m3())
    }
}

// 质量 ÷ 密度 = 体积：V = m/ρ
impl Div<Density> for Mass {
    type Output = Volume;
    fn div(self, rhs: Density) -> Self::Output {
        Volume::from_m3(self.as_kg() / rhs.as_kg_per_m3())
    }
}

impl Div<Density> for &Mass {
    type Output = Volume;
    fn div(self, rhs: Density) -> Self::Output {
        Volume::from_m3(self.as_kg() / rhs.as_kg_per_m3())
    }
}

impl<'a> Div<&'a Density> for Mass {
    type Output = Volume;
    fn div(self, rhs: &'a Density) -> Self::Output {
        Volume::from_m3(self.as_kg() / rhs.as_kg_per_m3())
    }
}

impl<'a, 'b> Div<&'b Density> for &'a Mass {
    type Output = Volume;
    fn div(self, rhs: &'b Density) -> Self::Output {
        Volume::from_m3(self.as_kg() / rhs.as_kg_per_m3())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let rho = Density::from_kg_per_m3(1000.0);
        assert_relative_eq!(rho.as_kg_per_m3(), 1000.0, epsilon = 1e-9);
        assert_relative_eq!(rho.as_g_per_cm3(), 1.0, epsilon = 1e-9);
        assert_relative_eq!(rho.as_g_per_liter(), 1000.0, epsilon = 1e-9);

        let rho = Density::from_g_per_cm3(1.0);
        assert_relative_eq!(rho.as_kg_per_m3(), 1000.0, epsilon = 1e-9);

        let rho = Density::from_g_per_liter(1.0);
        assert_relative_eq!(rho.as_kg_per_m3(), 1.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 三种构造表示的同一密度，所有取值方法必须一致
        let kg = Density::from_kg_per_m3(2700.0); // 铝
        let gcm3 = Density::from_g_per_cm3(2.7);
        let gl = Density::from_g_per_liter(2700.0);

        for rho in [kg, gcm3, gl] {
            assert_relative_eq!(rho.as_kg_per_m3(), 2700.0, epsilon = 1e-9);
            assert_relative_eq!(rho.as_g_per_cm3(), 2.7, epsilon = 1e-9);
            assert_relative_eq!(rho.as_g_per_liter(), 2700.0, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_g_per_cm3_vs_g_per_liter_differ_by_1000() {
        // 这是本类型最容易错的一处：两个「g/...」单位差 1000 倍
        let a = Density::from_g_per_cm3(1.0);
        let b = Density::from_g_per_liter(1.0);
        assert_relative_eq!(a.as_kg_per_m3() / b.as_kg_per_m3(), 1000.0, epsilon = 1e-9);

        // 水的密度：1 g/cm³ 或 1000 g/L
        assert_relative_eq!(a.as_kg_per_m3(), 1000.0, epsilon = 1e-9);
        assert_relative_eq!(
            Density::from_g_per_liter(1000.0).as_kg_per_m3(),
            1000.0,
            epsilon = 1e-9
        );
    }

    #[test]
    fn test_unit_round_trip() {
        for v in [0.0, 0.001, 1.225, 2700.0, 1e5] {
            let rho = Density::from_kg_per_m3(v);
            assert_relative_eq!(
                Density::from_g_per_cm3(rho.as_g_per_cm3()).as_kg_per_m3(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                Density::from_g_per_liter(rho.as_g_per_liter()).as_kg_per_m3(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = Density::from_kg_per_m3(1000.0);
        let b = Density::from_g_per_cm3(1.0); // 同为 1000 kg/m³

        assert_relative_eq!((a + b).as_kg_per_m3(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_kg_per_m3(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_kg_per_m3(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_kg_per_m3(), 2000.0, epsilon = 1e-9);

        assert_relative_eq!((a - b).as_kg_per_m3(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_kg_per_m3(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_kg_per_m3(), 0.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_kg_per_m3(), 0.0, epsilon = 1e-9);

        assert_relative_eq!((a * 2.0).as_kg_per_m3(), 2000.0, epsilon = 1e-9);
        assert_relative_eq!((a / 4.0).as_kg_per_m3(), 250.0, epsilon = 1e-9);
        assert_relative_eq!((0.5 * a).as_kg_per_m3(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((a * Coef::new(3.0)).as_kg_per_m3(), 3000.0, epsilon = 1e-9);
        assert_relative_eq!((a / Coef::new(2.0)).as_kg_per_m3(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((-a).as_kg_per_m3(), -1000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mul_volume_gives_mass() {
        let rho = Density::from_kg_per_m3(2700.0); // 铝
        let vol = Volume::from_m3(0.5);

        assert_relative_eq!((rho * vol).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((&rho * vol).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((rho * &vol).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((&rho * &vol).as_kg(), 1350.0, epsilon = 1e-9);

        // 交换律
        assert_relative_eq!((vol * rho).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((&vol * rho).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((vol * &rho).as_kg(), 1350.0, epsilon = 1e-9);
        assert_relative_eq!((&vol * &rho).as_kg(), 1350.0, epsilon = 1e-9);

        // 方法形式
        assert_relative_eq!(rho.mass(vol).as_kg(), 1350.0, epsilon = 1e-9);

        // 用 g/cm³ 构造的同一密度，结果必须相同
        let rho2 = Density::from_g_per_cm3(2.7);
        assert_relative_eq!((rho2 * vol).as_kg(), 1350.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mass_div_volume_gives_density() {
        let m = Mass::from_kg(1350.0);
        let vol = Volume::from_m3(0.5);
        let rho = m / vol;
        assert_relative_eq!(rho.as_kg_per_m3(), 2700.0, epsilon = 1e-9);
        assert_relative_eq!((&m / &vol).as_kg_per_m3(), 2700.0, epsilon = 1e-9);
        assert_relative_eq!((m / &vol).as_kg_per_m3(), 2700.0, epsilon = 1e-9);
        assert_relative_eq!((&m / vol).as_kg_per_m3(), 2700.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mass_div_density_gives_volume() {
        let m = Mass::from_kg(1350.0);
        let rho = Density::from_kg_per_m3(2700.0);
        let vol = m / rho;
        assert_relative_eq!(vol.as_m3(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((&m / &rho).as_m3(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((m / &rho).as_m3(), 0.5, epsilon = 1e-12);
        assert_relative_eq!((&m / rho).as_m3(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn test_mul_velocity_gives_mass_flux() {
        let rho = Density::from_kg_per_m3(1.225); // 海平面空气
        let v = Velocity::from_m_per_sec(200.0);

        let mf = rho * v;
        assert_relative_eq!(mf.as_kg_per_m2_s(), 245.0, epsilon = 1e-9);

        // 引用变体
        assert_relative_eq!((&rho * v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((rho * &v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((&rho * &v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);

        // 交换律
        assert_relative_eq!((v * rho).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((&v * rho).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((v * &rho).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
        assert_relative_eq!((&v * &rho).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);

        // 方法形式
        assert_relative_eq!(rho.mass_flux(v).as_kg_per_m2_s(), 245.0, epsilon = 1e-9);
    }

    #[test]
    fn test_dynamic_pressure_has_half() {
        // ½ρv² = 0.5 × 1.225 × 200² = 24500 Pa
        let rho = Density::from_kg_per_m3(1.225);
        let v = Velocity::from_m_per_sec(200.0);
        assert_relative_eq!(rho.dynamic_pressure(v).as_pa(), 24_500.0, epsilon = 1e-9);

        // 运算符给出的是 ρv²（不含 1/2），恰好是两倍
        let no_half = rho * v * v;
        assert_relative_eq!(no_half.as_pa(), 49_000.0, epsilon = 1e-9);
        assert_relative_eq!(
            no_half.as_pa() / rho.dynamic_pressure(v).as_pa(),
            2.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_drag_force() {
        // F = ½ρv²·C_D·A = 24500 × 1.2 × 2 = 58800 N
        let rho = Density::from_kg_per_m3(1.225);
        let v = Velocity::from_m_per_sec(200.0);

        let f = rho.drag_force(v, Coef::new(1.2), Area::from_m2(2.0));
        assert_relative_eq!(f.as_newton(), 58_800.0, epsilon = 1e-6);

        // C_D 减半 → 力减半
        let f2 = rho.drag_force(v, Coef::new(0.6), Area::from_m2(2.0));
        assert_relative_eq!(f2.as_newton(), 29_400.0, epsilon = 1e-6);

        // 速度翻倍 → 力变四倍（平方律）
        let f3 = rho.drag_force(
            Velocity::from_m_per_sec(400.0),
            Coef::new(1.2),
            Area::from_m2(2.0),
        );
        assert_relative_eq!(f3.as_newton(), 4.0 * 58_800.0, epsilon = 1e-4);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let rho = Density::default();
        assert!(rho.is_zero());
        assert_eq!(rho.default_unit_value(), 0.0);
        assert_eq!(rho.unit_display_name(), "Density (kg/m³)");

        let mut rho2 = Density::default();
        rho2.set_value(1.225);
        assert_relative_eq!(rho2.as_kg_per_m3(), 1.225, epsilon = 1e-12);
        assert!(!rho2.is_zero());

        assert!(rho2.as_any().downcast_ref::<Density>().is_some());
    }
}
