use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Area, Coef, Energy, Force, PhysicalQuantity, Pressure, PressureType, Volume,
};

/// `1 hPa` 等于多少 `Pa`。
const HPA_TO_PA: f64 = 1e2;
/// `1 kPa` 等于多少 `Pa`。
const KPA_TO_PA: f64 = 1e3;
/// `1 MPa` 等于多少 `Pa`。
const MPA_TO_PA: f64 = 1e6;
/// `1 bar` 等于多少 `Pa`。
///
/// 精确值 1e5。注意与 `atm` 差 1.3% —— 两者在手册里常被随手混用。
const BAR_TO_PA: f64 = 1e5;
/// `1 atm` 等于多少 `Pa`。
///
/// 定义值 101325，**不是 1e5**。`1 atm = 1.01325 bar`。
const ATM_TO_PA: f64 = 101_325.0;
/// `1 mmHg` 等于多少 `Pa`。
///
/// 约定值 133.322387415。与 `Torr`（= 101325/760 ≈ 133.322368）差 1.4e-7
/// 相对量，工程上视为相等，本库只提供 mmHg 一种。
const MMHG_TO_PA: f64 = 133.322_387_415;

impl Default for Pressure {
    fn default() -> Self {
        Self::from_pa(0.0)
    }
}

impl PhysicalQuantity for Pressure {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_pa()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "Pressure (Pa)".to_string()
    }
}

impl Pressure {
    pub fn from_pa(v: f64) -> Self {
        Self {
            default_type: PressureType::Pascal,
            v,
        }
    }

    pub fn from_hpa(v: f64) -> Self {
        Self {
            default_type: PressureType::HectoPascal,
            v,
        }
    }

    pub fn from_kpa(v: f64) -> Self {
        Self {
            default_type: PressureType::KiloPascal,
            v,
        }
    }

    pub fn from_mpa(v: f64) -> Self {
        Self {
            default_type: PressureType::MegaPascal,
            v,
        }
    }

    pub fn from_bar(v: f64) -> Self {
        Self {
            default_type: PressureType::Bar,
            v,
        }
    }

    pub fn from_atm(v: f64) -> Self {
        Self {
            default_type: PressureType::Atmosphere,
            v,
        }
    }

    pub fn from_mmhg(v: f64) -> Self {
        Self {
            default_type: PressureType::MilliMeterOfMercury,
            v,
        }
    }

    pub fn as_pa(&self) -> f64 {
        match self.default_type {
            PressureType::Pascal => self.v,
            PressureType::HectoPascal => self.v * HPA_TO_PA,
            PressureType::KiloPascal => self.v * KPA_TO_PA,
            PressureType::MegaPascal => self.v * MPA_TO_PA,
            PressureType::Bar => self.v * BAR_TO_PA,
            PressureType::Atmosphere => self.v * ATM_TO_PA,
            PressureType::MilliMeterOfMercury => self.v * MMHG_TO_PA,
        }
    }

    pub fn as_hpa(&self) -> f64 {
        self.as_pa() / HPA_TO_PA
    }

    pub fn as_kpa(&self) -> f64 {
        self.as_pa() / KPA_TO_PA
    }

    pub fn as_mpa(&self) -> f64 {
        self.as_pa() / MPA_TO_PA
    }

    pub fn as_bar(&self) -> f64 {
        self.as_pa() / BAR_TO_PA
    }

    pub fn as_atm(&self) -> f64 {
        self.as_pa() / ATM_TO_PA
    }

    pub fn as_mmhg(&self) -> f64 {
        self.as_pa() / MMHG_TO_PA
    }

    /// 作用在某个面积上的力：`F = P · A`。
    ///
    /// 光压推进时记得乘反射系数：`F = P · A · C_R`，`C_R` 用 [`Coef`] 传入。
    pub fn force_on(&self, area: Area) -> Force {
        Force::from_newton(self.as_pa() * area.as_m2())
    }

    /// 对外做功（等压过程）：`W = P · V`。
    pub fn work_on(&self, volume: Volume) -> Energy {
        Energy::from_joule(self.as_pa() * volume.as_m3())
    }
}

impl Add for Pressure {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_pa(self.as_pa() + rhs.as_pa())
    }
}

impl<'a, 'b> Add<&'b Pressure> for &'a Pressure {
    type Output = Pressure;
    fn add(self, rhs: &'b Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() + rhs.as_pa())
    }
}
impl<'a> Add<&'a Pressure> for Pressure {
    type Output = Pressure;
    fn add(self, rhs: &'a Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() + rhs.as_pa())
    }
}
impl<'a> Add<Pressure> for &'a Pressure {
    type Output = Pressure;
    fn add(self, rhs: Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() + rhs.as_pa())
    }
}

impl Sub for Pressure {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_pa(self.as_pa() - rhs.as_pa())
    }
}

impl<'a, 'b> Sub<&'b Pressure> for &'a Pressure {
    type Output = Pressure;
    fn sub(self, rhs: &'b Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() - rhs.as_pa())
    }
}
impl<'a> Sub<&'a Pressure> for Pressure {
    type Output = Pressure;
    fn sub(self, rhs: &'a Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() - rhs.as_pa())
    }
}
impl<'a> Sub<Pressure> for &'a Pressure {
    type Output = Pressure;
    fn sub(self, rhs: Pressure) -> Self::Output {
        Pressure::from_pa(self.as_pa() - rhs.as_pa())
    }
}

impl Neg for Pressure {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_pa(-self.as_pa())
    }
}

impl Mul<f64> for Pressure {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_pa(self.as_pa() * rhs)
    }
}

impl Div<f64> for Pressure {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_pa(self.as_pa() / rhs)
    }
}

impl Mul<Pressure> for f64 {
    type Output = Pressure;
    fn mul(self, rhs: Pressure) -> Self::Output {
        Pressure::from_pa(self * rhs.as_pa())
    }
}

impl Mul<Coef> for Pressure {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_pa(self.as_pa() * rhs.get_value())
    }
}

impl Div<Coef> for Pressure {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_pa(self.as_pa() / rhs.get_value())
    }
}

// 压力 × 面积 = 力：F = PA
impl Mul<Area> for Pressure {
    type Output = Force;
    fn mul(self, rhs: Area) -> Self::Output {
        Force::from_newton(self.as_pa() * rhs.as_m2())
    }
}

impl Mul<Area> for &Pressure {
    type Output = Force;
    fn mul(self, rhs: Area) -> Self::Output {
        Force::from_newton(self.as_pa() * rhs.as_m2())
    }
}

impl<'a> Mul<&'a Area> for Pressure {
    type Output = Force;
    fn mul(self, rhs: &'a Area) -> Self::Output {
        Force::from_newton(self.as_pa() * rhs.as_m2())
    }
}

impl<'a, 'b> Mul<&'b Area> for &'a Pressure {
    type Output = Force;
    fn mul(self, rhs: &'b Area) -> Self::Output {
        Force::from_newton(self.as_pa() * rhs.as_m2())
    }
}

// 面积 × 压力 = 力（交换律）
impl Mul<Pressure> for Area {
    type Output = Force;
    fn mul(self, rhs: Pressure) -> Self::Output {
        Force::from_newton(self.as_m2() * rhs.as_pa())
    }
}

impl<'a> Mul<Pressure> for &'a Area {
    type Output = Force;
    fn mul(self, rhs: Pressure) -> Self::Output {
        Force::from_newton(self.as_m2() * rhs.as_pa())
    }
}

impl<'a> Mul<&'a Pressure> for Area {
    type Output = Force;
    fn mul(self, rhs: &'a Pressure) -> Self::Output {
        Force::from_newton(self.as_m2() * rhs.as_pa())
    }
}

impl<'a, 'b> Mul<&'b Pressure> for &'a Area {
    type Output = Force;
    fn mul(self, rhs: &'b Pressure) -> Self::Output {
        Force::from_newton(self.as_m2() * rhs.as_pa())
    }
}

// 压力 × 体积 = 能量：W = PV
impl Mul<Volume> for Pressure {
    type Output = Energy;
    fn mul(self, rhs: Volume) -> Self::Output {
        Energy::from_joule(self.as_pa() * rhs.as_m3())
    }
}

impl Mul<Volume> for &Pressure {
    type Output = Energy;
    fn mul(self, rhs: Volume) -> Self::Output {
        Energy::from_joule(self.as_pa() * rhs.as_m3())
    }
}

impl<'a> Mul<&'a Volume> for Pressure {
    type Output = Energy;
    fn mul(self, rhs: &'a Volume) -> Self::Output {
        Energy::from_joule(self.as_pa() * rhs.as_m3())
    }
}

impl<'a, 'b> Mul<&'b Volume> for &'a Pressure {
    type Output = Energy;
    fn mul(self, rhs: &'b Volume) -> Self::Output {
        Energy::from_joule(self.as_pa() * rhs.as_m3())
    }
}

// 体积 × 压力 = 能量（交换律）
impl Mul<Pressure> for Volume {
    type Output = Energy;
    fn mul(self, rhs: Pressure) -> Self::Output {
        Energy::from_joule(self.as_m3() * rhs.as_pa())
    }
}

impl<'a> Mul<Pressure> for &'a Volume {
    type Output = Energy;
    fn mul(self, rhs: Pressure) -> Self::Output {
        Energy::from_joule(self.as_m3() * rhs.as_pa())
    }
}

impl<'a> Mul<&'a Pressure> for Volume {
    type Output = Energy;
    fn mul(self, rhs: &'a Pressure) -> Self::Output {
        Energy::from_joule(self.as_m3() * rhs.as_pa())
    }
}

impl<'a, 'b> Mul<&'b Pressure> for &'a Volume {
    type Output = Energy;
    fn mul(self, rhs: &'b Pressure) -> Self::Output {
        Energy::from_joule(self.as_m3() * rhs.as_pa())
    }
}

// 力 ÷ 面积 = 压力：P = F/A
impl Div<Area> for Force {
    type Output = Pressure;
    fn div(self, rhs: Area) -> Self::Output {
        Pressure::from_pa(self.as_newton() / rhs.as_m2())
    }
}

impl Div<Area> for &Force {
    type Output = Pressure;
    fn div(self, rhs: Area) -> Self::Output {
        Pressure::from_pa(self.as_newton() / rhs.as_m2())
    }
}

impl<'a> Div<&'a Area> for Force {
    type Output = Pressure;
    fn div(self, rhs: &'a Area) -> Self::Output {
        Pressure::from_pa(self.as_newton() / rhs.as_m2())
    }
}

impl<'a, 'b> Div<&'b Area> for &'a Force {
    type Output = Pressure;
    fn div(self, rhs: &'b Area) -> Self::Output {
        Pressure::from_pa(self.as_newton() / rhs.as_m2())
    }
}

// 能量 ÷ 体积 = 压力：P = W/V
impl Div<Volume> for Energy {
    type Output = Pressure;
    fn div(self, rhs: Volume) -> Self::Output {
        Pressure::from_pa(self.as_joule() / rhs.as_m3())
    }
}

impl Div<Volume> for &Energy {
    type Output = Pressure;
    fn div(self, rhs: Volume) -> Self::Output {
        Pressure::from_pa(self.as_joule() / rhs.as_m3())
    }
}

impl<'a> Div<&'a Volume> for Energy {
    type Output = Pressure;
    fn div(self, rhs: &'a Volume) -> Self::Output {
        Pressure::from_pa(self.as_joule() / rhs.as_m3())
    }
}

impl<'a, 'b> Div<&'b Volume> for &'a Energy {
    type Output = Pressure;
    fn div(self, rhs: &'b Volume) -> Self::Output {
        Pressure::from_pa(self.as_joule() / rhs.as_m3())
    }
}

// 能量 ÷ 压力 = 体积：V = W/P
impl Div<Pressure> for Energy {
    type Output = Volume;
    fn div(self, rhs: Pressure) -> Self::Output {
        Volume::from_m3(self.as_joule() / rhs.as_pa())
    }
}

impl Div<Pressure> for &Energy {
    type Output = Volume;
    fn div(self, rhs: Pressure) -> Self::Output {
        Volume::from_m3(self.as_joule() / rhs.as_pa())
    }
}

impl<'a> Div<&'a Pressure> for Energy {
    type Output = Volume;
    fn div(self, rhs: &'a Pressure) -> Self::Output {
        Volume::from_m3(self.as_joule() / rhs.as_pa())
    }
}

impl<'a, 'b> Div<&'b Pressure> for &'a Energy {
    type Output = Volume;
    fn div(self, rhs: &'b Pressure) -> Self::Output {
        Volume::from_m3(self.as_joule() / rhs.as_pa())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        assert_relative_eq!(Pressure::from_pa(101_325.0).as_atm(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(Pressure::from_atm(1.0).as_pa(), 101_325.0, epsilon = 1e-9);
        assert_relative_eq!(Pressure::from_bar(1.0).as_pa(), 100_000.0, epsilon = 1e-9);
        assert_relative_eq!(Pressure::from_kpa(1.0).as_pa(), 1000.0, epsilon = 1e-9);
        assert_relative_eq!(Pressure::from_mpa(1.0).as_pa(), 1e6, epsilon = 1e-3);
        assert_relative_eq!(Pressure::from_hpa(1013.25).as_pa(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!(Pressure::from_mmhg(760.0).as_pa(), 101_325.0, epsilon = 0.1);
    }

    #[test]
    fn test_bar_vs_atm_differ_by_1_3_percent() {
        // 这是本类型最容易错的一处：bar 与 atm 差 1.3%
        let bar = Pressure::from_bar(1.0);
        let atm = Pressure::from_atm(1.0);
        let ratio = atm.as_pa() / bar.as_pa();
        assert_relative_eq!(ratio, 1.01325, epsilon = 1e-9);

        // 换算成对方单位
        assert_relative_eq!(atm.as_bar(), 1.01325, epsilon = 1e-9);
        assert_relative_eq!(bar.as_atm(), 1.0 / 1.01325, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 除 mmHg 外的 6 种表示，彼此严格等价
        let p = 101_325.0;
        let variants = [
            Pressure::from_pa(p),
            Pressure::from_hpa(p / 100.0),
            Pressure::from_kpa(p / 1000.0),
            Pressure::from_mpa(p / 1e6),
            Pressure::from_bar(p / 1e5),
            Pressure::from_atm(1.0),
        ];

        for v in variants {
            assert_relative_eq!(v.as_pa(), p, epsilon = 1e-6);
            assert_relative_eq!(v.as_hpa(), 1013.25, epsilon = 1e-9);
            assert_relative_eq!(v.as_kpa(), 101.325, epsilon = 1e-9);
            assert_relative_eq!(v.as_mpa(), 0.101325, epsilon = 1e-12);
            assert_relative_eq!(v.as_bar(), 1.01325, epsilon = 1e-12);
            assert_relative_eq!(v.as_atm(), 1.0, epsilon = 1e-12);
            assert_relative_eq!(v.as_mmhg(), 760.0, epsilon = 1e-2);
        }
    }

    #[test]
    fn test_mmhg_is_not_exactly_atm_over_760() {
        // 这是一个真实的物理细节：760 mmHg ≠ 1 atm 的精确值。
        //
        // 1 atm 定义为 101325 Pa；而 mmHg 按约定值 133.322387415 Pa 定义，
        // 两者独立，760 × 133.322387415 = 101325.0144 Pa，比 1 atm 大 0.0144 Pa。
        // Torr 才定义为 1/760 atm（= 133.322368 Pa），与 mmHg 差 1.4e-7 相对量。
        //
        // 工程上视为相等，但若做高精度真空计算，这个差别是真实存在的。
        let mmhg_760 = Pressure::from_mmhg(760.0);
        let one_atm = Pressure::from_atm(1.0);

        let diff = mmhg_760.as_pa() - one_atm.as_pa();
        assert_relative_eq!(diff, 0.014_435_4, epsilon = 1e-6);
        assert!(diff > 0.0, "760 mmHg 略大于 1 atm");
    }

    #[test]
    fn test_arith() {
        let a = Pressure::from_kpa(100.0);
        let b = Pressure::from_bar(1.0); // 同为 100 kPa

        assert_relative_eq!((a + b).as_pa(), 200_000.0, epsilon = 1e-6);
        assert_relative_eq!((&a + &b).as_pa(), 200_000.0, epsilon = 1e-6);
        assert_relative_eq!((a + &b).as_pa(), 200_000.0, epsilon = 1e-6);
        assert_relative_eq!((&a + b).as_pa(), 200_000.0, epsilon = 1e-6);

        assert_relative_eq!((a - b).as_pa(), 0.0, epsilon = 1e-6);
        assert_relative_eq!((&a - &b).as_pa(), 0.0, epsilon = 1e-6);
        assert_relative_eq!((a - &b).as_pa(), 0.0, epsilon = 1e-6);
        assert_relative_eq!((&a - b).as_pa(), 0.0, epsilon = 1e-6);

        assert_relative_eq!((a * 2.0).as_pa(), 200_000.0, epsilon = 1e-6);
        assert_relative_eq!((a / 4.0).as_pa(), 25_000.0, epsilon = 1e-6);
        assert_relative_eq!((0.5 * a).as_pa(), 50_000.0, epsilon = 1e-6);
        assert_relative_eq!((a * Coef::new(3.0)).as_pa(), 300_000.0, epsilon = 1e-6);
        assert_relative_eq!((a / Coef::new(2.0)).as_pa(), 50_000.0, epsilon = 1e-6);
        assert_relative_eq!((-a).as_pa(), -100_000.0, epsilon = 1e-6);
    }

    #[test]
    fn test_mul_area_gives_force() {
        // 太阳光压（1 AU）作用在 10 m² 太阳翼上
        let p = Pressure::from_pa(4.56e-6);
        let a = Area::from_m2(10.0);

        let f = p * a;
        assert_relative_eq!(f.as_newton(), 4.56e-5, epsilon = 1e-15);

        assert_relative_eq!((&p * a).as_newton(), 4.56e-5, epsilon = 1e-15);
        assert_relative_eq!((p * &a).as_newton(), 4.56e-5, epsilon = 1e-15);
        assert_relative_eq!((&p * &a).as_newton(), 4.56e-5, epsilon = 1e-15);

        // 交换律
        assert_relative_eq!((a * p).as_newton(), 4.56e-5, epsilon = 1e-15);
        assert_relative_eq!((&a * p).as_newton(), 4.56e-5, epsilon = 1e-15);
        assert_relative_eq!((a * &p).as_newton(), 4.56e-5, epsilon = 1e-15);
        assert_relative_eq!((&a * &p).as_newton(), 4.56e-5, epsilon = 1e-15);

        // 方法形式
        assert_relative_eq!(p.force_on(a).as_newton(), 4.56e-5, epsilon = 1e-15);

        // 反射系数：C_R = 1.5（部分反射）→ 1.5 倍
        assert_relative_eq!(
            (p.force_on(a) * Coef::new(1.5)).as_newton(),
            6.84e-5,
            epsilon = 1e-15
        );
    }

    #[test]
    fn test_mul_volume_gives_energy() {
        // 1 atm × 1 m³ 等压膨胀 = 101325 J
        let p = Pressure::from_atm(1.0);
        let v = Volume::from_m3(1.0);

        assert_relative_eq!((p * v).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&p * v).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((p * &v).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&p * &v).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((v * p).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&v * p).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((v * &p).as_joule(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&v * &p).as_joule(), 101_325.0, epsilon = 1e-6);

        assert_relative_eq!(p.work_on(v).as_joule(), 101_325.0, epsilon = 1e-6);
    }

    #[test]
    fn test_force_div_area_gives_pressure() {
        let f = Force::from_newton(1000.0);
        let a = Area::from_m2(2.0);
        let p = f / a;
        assert_relative_eq!(p.as_pa(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((&f / &a).as_pa(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((f / &a).as_pa(), 500.0, epsilon = 1e-9);
        assert_relative_eq!((&f / a).as_pa(), 500.0, epsilon = 1e-9);
    }

    #[test]
    fn test_energy_div_volume_gives_pressure() {
        let e = Energy::from_joule(101_325.0);
        let v = Volume::from_m3(1.0);
        let p = e / v;
        assert_relative_eq!(p.as_pa(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&e / &v).as_pa(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((e / &v).as_pa(), 101_325.0, epsilon = 1e-6);
        assert_relative_eq!((&e / v).as_pa(), 101_325.0, epsilon = 1e-6);
    }

    #[test]
    fn test_energy_div_pressure_gives_volume() {
        let e = Energy::from_joule(101_325.0);
        let p = Pressure::from_atm(1.0);
        let v = e / p;
        assert_relative_eq!(v.as_m3(), 1.0, epsilon = 1e-9);
        assert_relative_eq!((&e / &p).as_m3(), 1.0, epsilon = 1e-9);
        assert_relative_eq!((e / &p).as_m3(), 1.0, epsilon = 1e-9);
        assert_relative_eq!((&e / p).as_m3(), 1.0, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let p = Pressure::default();
        assert!(p.is_zero());
        assert_eq!(p.default_unit_value(), 0.0);
        assert_eq!(p.unit_display_name(), "Pressure (Pa)");

        let mut p2 = Pressure::default();
        p2.set_value(101_325.0);
        assert_relative_eq!(p2.as_pa(), 101_325.0, epsilon = 1e-9);
        assert!(!p2.is_zero());

        assert!(p2.as_any().downcast_ref::<Pressure>().is_some());
    }
}
