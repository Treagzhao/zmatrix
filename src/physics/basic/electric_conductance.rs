use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricConductance, ElectricConductanceType, PhysicalQuantity};
use approx::assert_relative_eq;

impl Default for ElectricConductance {
    fn default() -> Self {
        Self::from_siemens(0.0)
    }
}

impl PhysicalQuantity for ElectricConductance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_siemens()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricConductance (S)".to_string()
    }
}

impl ElectricConductance {
    pub fn from_siemens(v: f64) -> Self {
        Self {
            default_type: ElectricConductanceType::Siemens,
            v,
        }
    }

    pub fn from_milli_siemens(v: f64) -> Self {
        Self {
            default_type: ElectricConductanceType::MilliSiemens,
            v,
        }
    }

    pub fn from_micro_siemens(v: f64) -> Self {
        Self {
            default_type: ElectricConductanceType::MicroSiemens,
            v,
        }
    }

    pub fn as_siemens(&self) -> f64 {
        match self.default_type {
            ElectricConductanceType::Siemens => self.v,
            ElectricConductanceType::MilliSiemens => self.v * 1e-3,
            ElectricConductanceType::MicroSiemens => self.v * 1e-6,
        }
    }

    pub fn as_milli_siemens(&self) -> f64 {
        match self.default_type {
            ElectricConductanceType::Siemens => self.v * 1e3,
            ElectricConductanceType::MilliSiemens => self.v,
            ElectricConductanceType::MicroSiemens => self.v * 1e-3,
        }
    }

    pub fn as_micro_siemens(&self) -> f64 {
        match self.default_type {
            ElectricConductanceType::Siemens => self.v * 1e6,
            ElectricConductanceType::MilliSiemens => self.v * 1000.0000000000001,
            ElectricConductanceType::MicroSiemens => self.v,
        }
    }

}

impl Add for ElectricConductance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_siemens() + rhs.as_siemens();
        Self::from_siemens(v)
    }
}

impl<'a, 'b> Add<&'b ElectricConductance> for &'a ElectricConductance {
    type Output = ElectricConductance;
    fn add(self, rhs: &'b ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() + rhs.as_siemens()) }
}
impl<'a> Add<&'a ElectricConductance> for ElectricConductance {
    type Output = ElectricConductance;
    fn add(self, rhs: &'a ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() + rhs.as_siemens()) }
}
impl<'a> Add<ElectricConductance> for &'a ElectricConductance {
    type Output = ElectricConductance;
    fn add(self, rhs: ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() + rhs.as_siemens()) }
}

impl Add<f64> for ElectricConductance {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricConductance { v, default_type: self.default_type }
    }
}

impl Sub for ElectricConductance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_siemens() - rhs.as_siemens();
        Self::from_siemens(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricConductance> for &'a ElectricConductance {
    type Output = ElectricConductance;
    fn sub(self, rhs: &'b ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() - rhs.as_siemens()) }
}
impl<'a> Sub<&'a ElectricConductance> for ElectricConductance {
    type Output = ElectricConductance;
    fn sub(self, rhs: &'a ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() - rhs.as_siemens()) }
}
impl<'a> Sub<ElectricConductance> for &'a ElectricConductance {
    type Output = ElectricConductance;
    fn sub(self, rhs: ElectricConductance) -> Self::Output { ElectricConductance::from_siemens(self.as_siemens() - rhs.as_siemens()) }
}

impl Sub<f64> for ElectricConductance {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricConductance { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricConductance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_siemens() * rhs;
        Self::from_siemens(v)
    }
}

impl Div<f64> for ElectricConductance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_siemens() / rhs;
        Self::from_siemens(v)
    }
}

impl Mul<ElectricConductance> for f64 {
    type Output = ElectricConductance;
    fn mul(self, rhs: ElectricConductance) -> Self::Output {
        let v = self * rhs.as_siemens();
        ElectricConductance::from_siemens(v)
    }
}

impl Div<ElectricConductance> for f64 {
    type Output = ElectricConductance;
    fn div(self, rhs: ElectricConductance) -> Self::Output {
        let v = self / rhs.as_siemens();
        ElectricConductance::from_siemens(v)
    }
}

impl Mul<Coef> for ElectricConductance {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_siemens() * rhs.get_value();
        Self::from_siemens(v)
    }
}

impl Div<Coef> for ElectricConductance {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_siemens() / rhs.get_value();
        Self::from_siemens(v)
    }
}

impl Neg for ElectricConductance {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_siemens();
        Self::from_siemens(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_conductance_create_and_convert() {
        let v1 = ElectricConductance::from_siemens(1.0);
        assert_relative_eq!(v1.as_siemens(), 1.0);
        let v2 = ElectricConductance::from_milli_siemens(1e3);
        assert_relative_eq!(v2.as_siemens(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_milli_siemens(), 1e3, epsilon = 1e-8);
        let v2 = ElectricConductance::from_micro_siemens(1e6);
        assert_relative_eq!(v2.as_siemens(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_micro_siemens(), 1e6, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_conductance_add_sub() {
        let a = ElectricConductance::from_siemens(5.0);
        let b = ElectricConductance::from_siemens(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_siemens(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_siemens(), 2.0);
    }

    #[test]
    fn test_electric_conductance_mul_div_f64() {
        let a = ElectricConductance::from_siemens(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_siemens(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_siemens(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_siemens(), 18.0);
    }

    #[test]
    fn test_electric_conductance_coef_ops() {
        let a = ElectricConductance::from_siemens(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_siemens(), 20.0);
        assert_relative_eq!((a / c).as_siemens(), 5.0);
    }

    #[test]
    fn test_electric_conductance_neg() {
        let a = ElectricConductance::from_siemens(5.0);
        let n = -a;
        assert_relative_eq!(n.as_siemens(), -5.0);
    }

    #[test]
    fn test_electric_conductance_default_is_zero() {
        let a = ElectricConductance::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_conductance_ref_ops() {
        let a = ElectricConductance::from_siemens(2.0);
        let b = ElectricConductance::from_siemens(3.0);
        assert_relative_eq!((&a + &b).as_siemens(), 5.0);
        assert_relative_eq!((&a - &b).as_siemens(), -1.0);
        assert_relative_eq!((a + &b).as_siemens(), 5.0);
        assert_relative_eq!((&a + b).as_siemens(), 5.0);
    }

    #[test]
    fn test_electric_conductance_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricConductance::from_siemens(1.23);
        let d = g.as_any().downcast_ref::<ElectricConductance>().unwrap();
        assert_relative_eq!(d.as_siemens(), 1.23);
    }

    #[test]
    fn test_electric_conductance_set_value() {
        let mut a = ElectricConductance::from_siemens(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_siemens(), 42.0);
    }
}
