use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricResistance, ElectricResistanceType, PhysicalQuantity};
use approx::assert_relative_eq;

impl Default for ElectricResistance {
    fn default() -> Self {
        Self::from_ohm(0.0)
    }
}

impl PhysicalQuantity for ElectricResistance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_ohm()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricResistance (Ohm)".to_string()
    }
}

impl ElectricResistance {
    pub fn from_ohm(v: f64) -> Self {
        Self {
            default_type: ElectricResistanceType::Ohm,
            v,
        }
    }

    pub fn from_mill_ohm(v: f64) -> Self {
        Self {
            default_type: ElectricResistanceType::MillOhm,
            v,
        }
    }

    pub fn from_kilo_ohm(v: f64) -> Self {
        Self {
            default_type: ElectricResistanceType::KiloOhm,
            v,
        }
    }

    pub fn from_mega_ohm(v: f64) -> Self {
        Self {
            default_type: ElectricResistanceType::MegaOhm,
            v,
        }
    }

    pub fn as_ohm(&self) -> f64 {
        match self.default_type {
            ElectricResistanceType::Ohm => self.v,
            ElectricResistanceType::MillOhm => self.v * 1e-3,
            ElectricResistanceType::KiloOhm => self.v * 1e3,
            ElectricResistanceType::MegaOhm => self.v * 1e6,
        }
    }

    pub fn as_mill_ohm(&self) -> f64 {
        match self.default_type {
            ElectricResistanceType::Ohm => self.v * 1e3,
            ElectricResistanceType::MillOhm => self.v,
            ElectricResistanceType::KiloOhm => self.v * 1e6,
            ElectricResistanceType::MegaOhm => self.v * 1e9,
        }
    }

    pub fn as_kilo_ohm(&self) -> f64 {
        match self.default_type {
            ElectricResistanceType::Ohm => self.v * 1e-3,
            ElectricResistanceType::MillOhm => self.v * 1e-6,
            ElectricResistanceType::KiloOhm => self.v,
            ElectricResistanceType::MegaOhm => self.v * 1e3,
        }
    }

    pub fn as_mega_ohm(&self) -> f64 {
        match self.default_type {
            ElectricResistanceType::Ohm => self.v * 1e-6,
            ElectricResistanceType::MillOhm => self.v * 1e-9,
            ElectricResistanceType::KiloOhm => self.v * 1e-3,
            ElectricResistanceType::MegaOhm => self.v,
        }
    }

}

impl Add for ElectricResistance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_ohm() + rhs.as_ohm();
        Self::from_ohm(v)
    }
}

impl<'a, 'b> Add<&'b ElectricResistance> for &'a ElectricResistance {
    type Output = ElectricResistance;
    fn add(self, rhs: &'b ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() + rhs.as_ohm()) }
}
impl<'a> Add<&'a ElectricResistance> for ElectricResistance {
    type Output = ElectricResistance;
    fn add(self, rhs: &'a ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() + rhs.as_ohm()) }
}
impl<'a> Add<ElectricResistance> for &'a ElectricResistance {
    type Output = ElectricResistance;
    fn add(self, rhs: ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() + rhs.as_ohm()) }
}

impl Add<f64> for ElectricResistance {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricResistance { v, default_type: self.default_type }
    }
}

impl Sub for ElectricResistance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_ohm() - rhs.as_ohm();
        Self::from_ohm(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricResistance> for &'a ElectricResistance {
    type Output = ElectricResistance;
    fn sub(self, rhs: &'b ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() - rhs.as_ohm()) }
}
impl<'a> Sub<&'a ElectricResistance> for ElectricResistance {
    type Output = ElectricResistance;
    fn sub(self, rhs: &'a ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() - rhs.as_ohm()) }
}
impl<'a> Sub<ElectricResistance> for &'a ElectricResistance {
    type Output = ElectricResistance;
    fn sub(self, rhs: ElectricResistance) -> Self::Output { ElectricResistance::from_ohm(self.as_ohm() - rhs.as_ohm()) }
}

impl Sub<f64> for ElectricResistance {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricResistance { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricResistance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_ohm() * rhs;
        Self::from_ohm(v)
    }
}

impl Div<f64> for ElectricResistance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_ohm() / rhs;
        Self::from_ohm(v)
    }
}

impl Mul<ElectricResistance> for f64 {
    type Output = ElectricResistance;
    fn mul(self, rhs: ElectricResistance) -> Self::Output {
        let v = self * rhs.as_ohm();
        ElectricResistance::from_ohm(v)
    }
}

impl Div<ElectricResistance> for f64 {
    type Output = ElectricResistance;
    fn div(self, rhs: ElectricResistance) -> Self::Output {
        let v = self / rhs.as_ohm();
        ElectricResistance::from_ohm(v)
    }
}

impl Mul<Coef> for ElectricResistance {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_ohm() * rhs.get_value();
        Self::from_ohm(v)
    }
}

impl Div<Coef> for ElectricResistance {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_ohm() / rhs.get_value();
        Self::from_ohm(v)
    }
}

impl Neg for ElectricResistance {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_ohm();
        Self::from_ohm(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_resistance_create_and_convert() {
        let v1 = ElectricResistance::from_ohm(1.0);
        assert_relative_eq!(v1.as_ohm(), 1.0);
        let v2 = ElectricResistance::from_mill_ohm(1e3);
        assert_relative_eq!(v2.as_ohm(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_mill_ohm(), 1e3, epsilon = 1e-8);
        let v2 = ElectricResistance::from_kilo_ohm(1e-3);
        assert_relative_eq!(v2.as_ohm(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_kilo_ohm(), 1e-3, epsilon = 1e-8);
        let v2 = ElectricResistance::from_mega_ohm(1e-6);
        assert_relative_eq!(v2.as_ohm(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_mega_ohm(), 1e-6, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_resistance_add_sub() {
        let a = ElectricResistance::from_ohm(5.0);
        let b = ElectricResistance::from_ohm(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_ohm(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_ohm(), 2.0);
    }

    #[test]
    fn test_electric_resistance_mul_div_f64() {
        let a = ElectricResistance::from_ohm(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_ohm(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_ohm(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_ohm(), 18.0);
    }

    #[test]
    fn test_electric_resistance_coef_ops() {
        let a = ElectricResistance::from_ohm(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_ohm(), 20.0);
        assert_relative_eq!((a / c).as_ohm(), 5.0);
    }

    #[test]
    fn test_electric_resistance_neg() {
        let a = ElectricResistance::from_ohm(5.0);
        let n = -a;
        assert_relative_eq!(n.as_ohm(), -5.0);
    }

    #[test]
    fn test_electric_resistance_default_is_zero() {
        let a = ElectricResistance::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_resistance_ref_ops() {
        let a = ElectricResistance::from_ohm(2.0);
        let b = ElectricResistance::from_ohm(3.0);
        assert_relative_eq!((&a + &b).as_ohm(), 5.0);
        assert_relative_eq!((&a - &b).as_ohm(), -1.0);
        assert_relative_eq!((a + &b).as_ohm(), 5.0);
        assert_relative_eq!((&a + b).as_ohm(), 5.0);
    }

    #[test]
    fn test_electric_resistance_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricResistance::from_ohm(1.23);
        let d = g.as_any().downcast_ref::<ElectricResistance>().unwrap();
        assert_relative_eq!(d.as_ohm(), 1.23);
    }

    #[test]
    fn test_electric_resistance_set_value() {
        let mut a = ElectricResistance::from_ohm(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_ohm(), 42.0);
    }

    #[test]
    fn test_electric_resistance_all_as_branches() {
        let mo = ElectricResistance::from_mill_ohm(1.0);
        let ko = ElectricResistance::from_kilo_ohm(1.0);
        let mo2 = ElectricResistance::from_mega_ohm(1.0);

        // as_mill_ohm 非默认分支
        assert_relative_eq!(mo.as_mill_ohm(), 1.0);
        assert_relative_eq!(ko.as_mill_ohm(), 1e6);
        assert_relative_eq!(mo2.as_mill_ohm(), 1e9);

        // as_kilo_ohm 非默认分支
        assert_relative_eq!(mo.as_kilo_ohm(), 1e-6);
        assert_relative_eq!(ko.as_kilo_ohm(), 1.0);
        assert_relative_eq!(mo2.as_kilo_ohm(), 1e3);

        // as_mega_ohm 非默认分支
        assert_relative_eq!(mo.as_mega_ohm(), 1e-9);
        assert_relative_eq!(ko.as_mega_ohm(), 1e-3);
        assert_relative_eq!(mo2.as_mega_ohm(), 1.0);
    }

    #[test]
    fn test_electric_resistance_f64_and_ref_sub() {
        let a = ElectricResistance::from_ohm(10.0);
        let b = ElectricResistance::from_ohm(3.0);
        // Add<f64>
        assert_relative_eq!((a + 5.0).as_ohm(), 15.0);
        // Sub<f64>
        assert_relative_eq!((a - 5.0).as_ohm(), 5.0);
        // Sub 引用形式
        assert_relative_eq!((a - &b).as_ohm(), 7.0);
        assert_relative_eq!((&a - b).as_ohm(), 7.0);
        // f64 / 量
        assert_relative_eq!((100.0 / a).as_ohm(), 10.0);
    }
}
