use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{
    Area, Coef, HeatFlux, HeatFluxType, PhysicalQuantity, Power,
};

/// `1 mW/cm²` 等于多少 `W/m²`。
///
/// 不是 1e-3 —— 分母是 cm² 而非法 m²：`1 mW / 1 cm² = 1e-3 W / 1e-4 m² = 10 W/m²`。
/// 这个 10 倍关系是单位换算里最容易写错的一处。
const MW_PER_CM2_TO_W_PER_M2: f64 = 10.0;

impl Default for HeatFlux {
    fn default() -> Self {
        Self::from_w_per_m2(0.0)
    }
}

impl PhysicalQuantity for HeatFlux {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_w_per_m2()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }

    fn unit_display_name(&self) -> String {
        "HeatFlux (W/m²)".to_string()
    }
}

impl HeatFlux {
    pub fn from_w_per_m2(v: f64) -> Self {
        Self {
            default_type: HeatFluxType::WattPerSquareMeter,
            v,
        }
    }

    pub fn from_kw_per_m2(v: f64) -> Self {
        Self {
            default_type: HeatFluxType::KiloWattPerSquareMeter,
            v,
        }
    }

    pub fn from_mw_per_cm2(v: f64) -> Self {
        Self {
            default_type: HeatFluxType::MilliWattPerSquareCentiMeter,
            v,
        }
    }

    pub fn as_w_per_m2(&self) -> f64 {
        match self.default_type {
            HeatFluxType::WattPerSquareMeter => self.v,
            HeatFluxType::KiloWattPerSquareMeter => self.v * 1e3,
            HeatFluxType::MilliWattPerSquareCentiMeter => self.v * MW_PER_CM2_TO_W_PER_M2,
        }
    }

    pub fn as_kw_per_m2(&self) -> f64 {
        match self.default_type {
            HeatFluxType::WattPerSquareMeter => self.v * 1e-3,
            HeatFluxType::KiloWattPerSquareMeter => self.v,
            HeatFluxType::MilliWattPerSquareCentiMeter => {
                self.v * MW_PER_CM2_TO_W_PER_M2 * 1e-3
            }
        }
    }

    pub fn as_mw_per_cm2(&self) -> f64 {
        match self.default_type {
            HeatFluxType::WattPerSquareMeter => self.v / MW_PER_CM2_TO_W_PER_M2,
            HeatFluxType::KiloWattPerSquareMeter => {
                self.v * 1e3 / MW_PER_CM2_TO_W_PER_M2
            }
            HeatFluxType::MilliWattPerSquareCentiMeter => self.v,
        }
    }
}

impl Add for HeatFlux {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() + rhs.as_w_per_m2())
    }
}

impl<'a, 'b> Add<&'b HeatFlux> for &'a HeatFlux {
    type Output = HeatFlux;
    fn add(self, rhs: &'b HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() + rhs.as_w_per_m2())
    }
}
impl<'a> Add<&'a HeatFlux> for HeatFlux {
    type Output = HeatFlux;
    fn add(self, rhs: &'a HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() + rhs.as_w_per_m2())
    }
}
impl<'a> Add<HeatFlux> for &'a HeatFlux {
    type Output = HeatFlux;
    fn add(self, rhs: HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() + rhs.as_w_per_m2())
    }
}

impl Sub for HeatFlux {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() - rhs.as_w_per_m2())
    }
}

impl<'a, 'b> Sub<&'b HeatFlux> for &'a HeatFlux {
    type Output = HeatFlux;
    fn sub(self, rhs: &'b HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() - rhs.as_w_per_m2())
    }
}
impl<'a> Sub<&'a HeatFlux> for HeatFlux {
    type Output = HeatFlux;
    fn sub(self, rhs: &'a HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() - rhs.as_w_per_m2())
    }
}
impl<'a> Sub<HeatFlux> for &'a HeatFlux {
    type Output = HeatFlux;
    fn sub(self, rhs: HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self.as_w_per_m2() - rhs.as_w_per_m2())
    }
}

impl Neg for HeatFlux {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_w_per_m2(-self.as_w_per_m2())
    }
}

impl Mul<f64> for HeatFlux {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() * rhs)
    }
}

impl Div<f64> for HeatFlux {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() / rhs)
    }
}

impl Mul<HeatFlux> for f64 {
    type Output = HeatFlux;
    fn mul(self, rhs: HeatFlux) -> Self::Output {
        HeatFlux::from_w_per_m2(self * rhs.as_w_per_m2())
    }
}

impl Div<HeatFlux> for f64 {
    type Output = Coef;
    fn div(self, rhs: HeatFlux) -> Self::Output {
        Coef::new(self / rhs.as_w_per_m2())
    }
}

impl Mul<Coef> for HeatFlux {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() * rhs.get_value())
    }
}

impl Div<Coef> for HeatFlux {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_w_per_m2(self.as_w_per_m2() / rhs.get_value())
    }
}

// 热流密度 × 面积 = 热功率：(W/m²) × m² = W
//
// 这是热流密度最有用的一个运算：表面上的载荷换成整块板要排掉的总热功率。
impl Mul<Area> for HeatFlux {
    type Output = Power;
    fn mul(self, rhs: Area) -> Self::Output {
        Power::from_watt(self.as_w_per_m2() * rhs.as_m2())
    }
}

impl Mul<Area> for &HeatFlux {
    type Output = Power;
    fn mul(self, rhs: Area) -> Self::Output {
        Power::from_watt(self.as_w_per_m2() * rhs.as_m2())
    }
}

impl<'a> Mul<&'a Area> for HeatFlux {
    type Output = Power;
    fn mul(self, rhs: &'a Area) -> Self::Output {
        Power::from_watt(self.as_w_per_m2() * rhs.as_m2())
    }
}

impl<'a, 'b> Mul<&'b Area> for &'a HeatFlux {
    type Output = Power;
    fn mul(self, rhs: &'b Area) -> Self::Output {
        Power::from_watt(self.as_w_per_m2() * rhs.as_m2())
    }
}

// 面积 × 热流密度 = 热功率（交换律，从面积一侧）
impl Mul<HeatFlux> for Area {
    type Output = Power;
    fn mul(self, rhs: HeatFlux) -> Self::Output {
        Power::from_watt(self.as_m2() * rhs.as_w_per_m2())
    }
}

impl<'a> Mul<HeatFlux> for &'a Area {
    type Output = Power;
    fn mul(self, rhs: HeatFlux) -> Self::Output {
        Power::from_watt(self.as_m2() * rhs.as_w_per_m2())
    }
}

impl<'a> Mul<&'a HeatFlux> for Area {
    type Output = Power;
    fn mul(self, rhs: &'a HeatFlux) -> Self::Output {
        Power::from_watt(self.as_m2() * rhs.as_w_per_m2())
    }
}

impl<'a, 'b> Mul<&'b HeatFlux> for &'a Area {
    type Output = Power;
    fn mul(self, rhs: &'b HeatFlux) -> Self::Output {
        Power::from_watt(self.as_m2() * rhs.as_w_per_m2())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        let q = HeatFlux::from_w_per_m2(1361.0);
        assert_relative_eq!(q.as_w_per_m2(), 1361.0, epsilon = 1e-9);
        assert_relative_eq!(q.as_kw_per_m2(), 1.361, epsilon = 1e-9);

        // 1 mW/cm² = 10 W/m²，不是 1e-3
        let q = HeatFlux::from_mw_per_cm2(1.0);
        assert_relative_eq!(q.as_w_per_m2(), 10.0, epsilon = 1e-9);
        assert_relative_eq!(q.as_kw_per_m2(), 0.01, epsilon = 1e-9);

        // 反向
        let q = HeatFlux::from_w_per_m2(100.0);
        assert_relative_eq!(q.as_mw_per_cm2(), 10.0, epsilon = 1e-9);

        let q = HeatFlux::from_kw_per_m2(2.5);
        assert_relative_eq!(q.as_w_per_m2(), 2500.0, epsilon = 1e-9);
        assert_relative_eq!(q.as_mw_per_cm2(), 250.0, epsilon = 1e-9);
    }

    #[test]
    fn test_unit_round_trip() {
        // 任意单位构造，三种表示互转一圈都能回到原值
        for w in [0.0, 1.0, 10.0, 1361.0, 1e6] {
            let q = HeatFlux::from_w_per_m2(w);
            assert_relative_eq!(
                HeatFlux::from_kw_per_m2(q.as_kw_per_m2()).as_w_per_m2(),
                w,
                epsilon = 1e-6
            );
            assert_relative_eq!(
                HeatFlux::from_mw_per_cm2(q.as_mw_per_cm2()).as_w_per_m2(),
                w,
                epsilon = 1e-6
            );
        }
    }

    #[test]
    fn test_unit_independence_in_arith() {
        // 不同单位构造的同一天热流密度，运算结果必须一致
        let a = HeatFlux::from_w_per_m2(100.0);
        let b = HeatFlux::from_mw_per_cm2(10.0); // = 100 W/m²
        assert_relative_eq!(a.as_w_per_m2(), b.as_w_per_m2(), epsilon = 1e-9);

        assert_relative_eq!((a + b).as_w_per_m2(), 200.0, epsilon = 1e-9);
        assert_relative_eq!((a - b).as_w_per_m2(), 0.0, epsilon = 1e-9);

        // 混合单位求和：1361 + 230 (W/m²) 形式的反照叠加
        let sun = HeatFlux::from_w_per_m2(1361.0);
        let ir = HeatFlux::from_mw_per_cm2(23.0); // 230 W/m²
        assert_relative_eq!((sun + ir).as_w_per_m2(), 1591.0, epsilon = 1e-9);
    }

    #[test]
    fn test_scalar_arith() {
        let q = HeatFlux::from_w_per_m2(100.0);

        assert_relative_eq!((q * 2.0).as_w_per_m2(), 200.0, epsilon = 1e-9);
        assert_relative_eq!((q / 4.0).as_w_per_m2(), 25.0, epsilon = 1e-9);
        assert_relative_eq!((0.5 * q).as_w_per_m2(), 50.0, epsilon = 1e-9);
        assert_relative_eq!((q * Coef::new(3.0)).as_w_per_m2(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((q / Coef::new(2.0)).as_w_per_m2(), 50.0, epsilon = 1e-9);
        assert_relative_eq!((-q).as_w_per_m2(), -100.0, epsilon = 1e-9);
        assert_relative_eq!((200.0 / q).get_value(), 2.0, epsilon = 1e-9);
    }

    #[test]
    fn test_mul_area_gives_power() {
        // 太阳常数 × 1 m² = 1361 W
        let q = HeatFlux::from_w_per_m2(1361.0);
        let area = Area::from_m2(1.0);
        let p = q * area;
        assert_relative_eq!(p.as_watt(), 1361.0, epsilon = 1e-9);

        // 2 m² 太阳翼
        let area2 = Area::from_m2(2.0);
        assert_relative_eq!((q * area2).as_watt(), 2722.0, epsilon = 1e-9);

        // 交换律：面积 × 热流密度
        assert_relative_eq!((area2 * q).as_watt(), 2722.0, epsilon = 1e-9);

        // 引用变体
        assert_relative_eq!((&q * area2).as_watt(), 2722.0, epsilon = 1e-9);
        assert_relative_eq!((q * &area2).as_watt(), 2722.0, epsilon = 1e-9);
        assert_relative_eq!((&q * &area2).as_watt(), 2722.0, epsilon = 1e-9);
        assert_relative_eq!((&area2 * q).as_watt(), 2722.0, epsilon = 1e-9);
        assert_relative_eq!((area2 * &q).as_watt(), 2722.0, epsilon = 1e-9);
        assert_relative_eq!((&area2 * &q).as_watt(), 2722.0, epsilon = 1e-9);

        // 用 mW/cm² 构造的同一热流密度，乘以面积结果相同
        let q2 = HeatFlux::from_mw_per_cm2(136.1); // = 1361 W/m²
        assert_relative_eq!((q2 * area2).as_watt(), 2722.0, epsilon = 1e-9);
    }

    #[test]
    fn test_ref_variants() {
        let a = HeatFlux::from_w_per_m2(100.0);
        let b = HeatFlux::from_w_per_m2(40.0);

        // 加法 4 变体
        assert_relative_eq!((a + b).as_w_per_m2(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((&a + &b).as_w_per_m2(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((a + &b).as_w_per_m2(), 140.0, epsilon = 1e-9);
        assert_relative_eq!((&a + b).as_w_per_m2(), 140.0, epsilon = 1e-9);

        // 减法 4 变体
        assert_relative_eq!((a - b).as_w_per_m2(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((&a - &b).as_w_per_m2(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((a - &b).as_w_per_m2(), 60.0, epsilon = 1e-9);
        assert_relative_eq!((&a - b).as_w_per_m2(), 60.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 覆盖每个 as_* 里的三个 match 分支
        let w = HeatFlux::from_w_per_m2(100.0);
        let kw = HeatFlux::from_kw_per_m2(0.1); // 同为 100 W/m²
        let mw = HeatFlux::from_mw_per_cm2(10.0); // 同为 100 W/m²

        for q in [w, kw, mw] {
            assert_relative_eq!(q.as_w_per_m2(), 100.0, epsilon = 1e-9);
            assert_relative_eq!(q.as_kw_per_m2(), 0.1, epsilon = 1e-9);
            assert_relative_eq!(q.as_mw_per_cm2(), 10.0, epsilon = 1e-9);
        }

        // kW/m² 视图下再取 mW/cm²（跨分支换算）
        let q = HeatFlux::from_kw_per_m2(1.0); // 1000 W/m²
        assert_relative_eq!(q.as_mw_per_cm2(), 100.0, epsilon = 1e-9);

        // mW/cm² 视图下再取 kW/m²
        let q = HeatFlux::from_mw_per_cm2(50.0); // 500 W/m²
        assert_relative_eq!(q.as_kw_per_m2(), 0.5, epsilon = 1e-9);
    }

    #[test]
    fn test_area_mul_all_ref_variants() {
        let q = HeatFlux::from_w_per_m2(100.0);
        let a = Area::from_m2(3.0);

        // HeatFlux * Area 的 4 个变体
        assert_relative_eq!((q * a).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((&q * a).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((q * &a).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((&q * &a).as_watt(), 300.0, epsilon = 1e-9);

        // Area * HeatFlux 的 4 个变体
        assert_relative_eq!((a * q).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((&a * q).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((a * &q).as_watt(), 300.0, epsilon = 1e-9);
        assert_relative_eq!((&a * &q).as_watt(), 300.0, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let q = HeatFlux::default();
        assert_relative_eq!(q.as_w_per_m2(), 0.0, epsilon = 1e-12);
        assert!(q.is_zero());
        assert_eq!(q.default_unit_value(), 0.0);
        assert_eq!(q.unit_display_name(), "HeatFlux (W/m²)");

        let mut q2 = HeatFlux::default();
        q2.set_value(500.0);
        assert_relative_eq!(q2.as_w_per_m2(), 500.0, epsilon = 1e-12);
        assert!(!q2.is_zero());

        let any = q2.as_any();
        assert!(any.downcast_ref::<HeatFlux>().is_some());
    }
}
