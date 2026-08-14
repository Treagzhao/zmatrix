use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricCharge, ElectricChargeType, PhysicalQuantity};
use approx::assert_relative_eq;

impl Default for ElectricCharge {
    fn default() -> Self {
        Self::from_coulomb(0.0)
    }
}

impl PhysicalQuantity for ElectricCharge {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_coulomb()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricCharge (C)".to_string()
    }
}

impl ElectricCharge {
    pub fn from_coulomb(v: f64) -> Self {
        Self {
            default_type: ElectricChargeType::Coulomb,
            v,
        }
    }

    pub fn from_milli_coulomb(v: f64) -> Self {
        Self {
            default_type: ElectricChargeType::MilliCoulomb,
            v,
        }
    }

    pub fn from_micro_coulomb(v: f64) -> Self {
        Self {
            default_type: ElectricChargeType::MicroCoulomb,
            v,
        }
    }

    pub fn from_nano_coulomb(v: f64) -> Self {
        Self {
            default_type: ElectricChargeType::NanoCoulomb,
            v,
        }
    }

    pub fn as_coulomb(&self) -> f64 {
        match self.default_type {
            ElectricChargeType::Coulomb => self.v,
            ElectricChargeType::MilliCoulomb => self.v * 1e-3,
            ElectricChargeType::MicroCoulomb => self.v * 1e-6,
            ElectricChargeType::NanoCoulomb => self.v * 1e-9,
        }
    }

    pub fn as_milli_coulomb(&self) -> f64 {
        match self.default_type {
            ElectricChargeType::Coulomb => self.v * 1e3,
            ElectricChargeType::MilliCoulomb => self.v,
            ElectricChargeType::MicroCoulomb => self.v * 1e-3,
            ElectricChargeType::NanoCoulomb => self.v * 1e-6,
        }
    }

    pub fn as_micro_coulomb(&self) -> f64 {
        match self.default_type {
            ElectricChargeType::Coulomb => self.v * 1e6,
            ElectricChargeType::MilliCoulomb => self.v * 1000.0000000000001,
            ElectricChargeType::MicroCoulomb => self.v,
            ElectricChargeType::NanoCoulomb => self.v * 1e-3,
        }
    }

    pub fn as_nano_coulomb(&self) -> f64 {
        match self.default_type {
            ElectricChargeType::Coulomb => self.v * 999999999.9999999,
            ElectricChargeType::MilliCoulomb => self.v * 1e6,
            ElectricChargeType::MicroCoulomb => self.v * 999.9999999999999,
            ElectricChargeType::NanoCoulomb => self.v,
        }
    }

}

impl Add for ElectricCharge {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_coulomb() + rhs.as_coulomb();
        Self::from_coulomb(v)
    }
}

impl<'a, 'b> Add<&'b ElectricCharge> for &'a ElectricCharge {
    type Output = ElectricCharge;
    fn add(self, rhs: &'b ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() + rhs.as_coulomb()) }
}
impl<'a> Add<&'a ElectricCharge> for ElectricCharge {
    type Output = ElectricCharge;
    fn add(self, rhs: &'a ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() + rhs.as_coulomb()) }
}
impl<'a> Add<ElectricCharge> for &'a ElectricCharge {
    type Output = ElectricCharge;
    fn add(self, rhs: ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() + rhs.as_coulomb()) }
}

impl Add<f64> for ElectricCharge {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricCharge { v, default_type: self.default_type }
    }
}

impl Sub for ElectricCharge {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_coulomb() - rhs.as_coulomb();
        Self::from_coulomb(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricCharge> for &'a ElectricCharge {
    type Output = ElectricCharge;
    fn sub(self, rhs: &'b ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() - rhs.as_coulomb()) }
}
impl<'a> Sub<&'a ElectricCharge> for ElectricCharge {
    type Output = ElectricCharge;
    fn sub(self, rhs: &'a ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() - rhs.as_coulomb()) }
}
impl<'a> Sub<ElectricCharge> for &'a ElectricCharge {
    type Output = ElectricCharge;
    fn sub(self, rhs: ElectricCharge) -> Self::Output { ElectricCharge::from_coulomb(self.as_coulomb() - rhs.as_coulomb()) }
}

impl Sub<f64> for ElectricCharge {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricCharge { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricCharge {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_coulomb() * rhs;
        Self::from_coulomb(v)
    }
}

impl Div<f64> for ElectricCharge {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_coulomb() / rhs;
        Self::from_coulomb(v)
    }
}

impl Mul<ElectricCharge> for f64 {
    type Output = ElectricCharge;
    fn mul(self, rhs: ElectricCharge) -> Self::Output {
        let v = self * rhs.as_coulomb();
        ElectricCharge::from_coulomb(v)
    }
}

impl Div<ElectricCharge> for f64 {
    type Output = ElectricCharge;
    fn div(self, rhs: ElectricCharge) -> Self::Output {
        let v = self / rhs.as_coulomb();
        ElectricCharge::from_coulomb(v)
    }
}

impl Mul<Coef> for ElectricCharge {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_coulomb() * rhs.get_value();
        Self::from_coulomb(v)
    }
}

impl Div<Coef> for ElectricCharge {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_coulomb() / rhs.get_value();
        Self::from_coulomb(v)
    }
}

impl Neg for ElectricCharge {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_coulomb();
        Self::from_coulomb(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_charge_create_and_convert() {
        let v1 = ElectricCharge::from_coulomb(1.0);
        assert_relative_eq!(v1.as_coulomb(), 1.0);
        let v2 = ElectricCharge::from_milli_coulomb(1e3);
        assert_relative_eq!(v2.as_coulomb(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_milli_coulomb(), 1e3, epsilon = 1e-8);
        let v2 = ElectricCharge::from_micro_coulomb(1e6);
        assert_relative_eq!(v2.as_coulomb(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_micro_coulomb(), 1e6, epsilon = 1e-8);
        let v2 = ElectricCharge::from_nano_coulomb(999999999.9999999);
        assert_relative_eq!(v2.as_coulomb(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_nano_coulomb(), 999999999.9999999, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_charge_add_sub() {
        let a = ElectricCharge::from_coulomb(5.0);
        let b = ElectricCharge::from_coulomb(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_coulomb(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_coulomb(), 2.0);
    }

    #[test]
    fn test_electric_charge_mul_div_f64() {
        let a = ElectricCharge::from_coulomb(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_coulomb(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_coulomb(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_coulomb(), 18.0);
    }

    #[test]
    fn test_electric_charge_coef_ops() {
        let a = ElectricCharge::from_coulomb(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_coulomb(), 20.0);
        assert_relative_eq!((a / c).as_coulomb(), 5.0);
    }

    #[test]
    fn test_electric_charge_neg() {
        let a = ElectricCharge::from_coulomb(5.0);
        let n = -a;
        assert_relative_eq!(n.as_coulomb(), -5.0);
    }

    #[test]
    fn test_electric_charge_default_is_zero() {
        let a = ElectricCharge::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_charge_ref_ops() {
        let a = ElectricCharge::from_coulomb(2.0);
        let b = ElectricCharge::from_coulomb(3.0);
        assert_relative_eq!((&a + &b).as_coulomb(), 5.0);
        assert_relative_eq!((&a - &b).as_coulomb(), -1.0);
        assert_relative_eq!((a + &b).as_coulomb(), 5.0);
        assert_relative_eq!((&a + b).as_coulomb(), 5.0);
    }

    #[test]
    fn test_electric_charge_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricCharge::from_coulomb(1.23);
        let d = g.as_any().downcast_ref::<ElectricCharge>().unwrap();
        assert_relative_eq!(d.as_coulomb(), 1.23);
    }

    #[test]
    fn test_electric_charge_set_value() {
        let mut a = ElectricCharge::from_coulomb(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_coulomb(), 42.0);
    }

    #[test]
    fn test_electric_charge_all_as_branches() {
        let mc = ElectricCharge::from_milli_coulomb(1.0);
        let uc = ElectricCharge::from_micro_coulomb(1.0);
        let nc = ElectricCharge::from_nano_coulomb(1.0);

        // as_milli_coulomb 非默认分支
        assert_relative_eq!(mc.as_milli_coulomb(), 1.0);
        assert_relative_eq!(uc.as_milli_coulomb(), 1e-3);
        assert_relative_eq!(nc.as_milli_coulomb(), 1e-6);

        // as_micro_coulomb 非默认分支
        assert_relative_eq!(mc.as_micro_coulomb(), 1000.0000000000001);
        assert_relative_eq!(uc.as_micro_coulomb(), 1.0);
        assert_relative_eq!(nc.as_micro_coulomb(), 1e-3);

        // as_nano_coulomb 非默认分支
        assert_relative_eq!(mc.as_nano_coulomb(), 1e6);
        assert_relative_eq!(uc.as_nano_coulomb(), 999.9999999999999);
        assert_relative_eq!(nc.as_nano_coulomb(), 1.0);
    }

    #[test]
    fn test_electric_charge_f64_and_ref_sub() {
        let a = ElectricCharge::from_coulomb(10.0);
        let b = ElectricCharge::from_coulomb(3.0);
        // Add<f64>
        assert_relative_eq!((a + 5.0).as_coulomb(), 15.0);
        // Sub<f64>
        assert_relative_eq!((a - 5.0).as_coulomb(), 5.0);
        // Sub 引用形式
        assert_relative_eq!((a - &b).as_coulomb(), 7.0);
        assert_relative_eq!((&a - b).as_coulomb(), 7.0);
        // f64 / 量
        assert_relative_eq!((100.0 / a).as_coulomb(), 10.0);
    }
}
