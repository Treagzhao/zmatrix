use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricCapacitance, ElectricCapacitanceType, PhysicalQuantity};
use approx::assert_relative_eq;

impl Default for ElectricCapacitance {
    fn default() -> Self {
        Self::from_farad(0.0)
    }
}

impl PhysicalQuantity for ElectricCapacitance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_farad()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricCapacitance (F)".to_string()
    }
}

impl ElectricCapacitance {
    pub fn from_farad(v: f64) -> Self {
        Self {
            default_type: ElectricCapacitanceType::Farad,
            v,
        }
    }

    pub fn from_milli_farad(v: f64) -> Self {
        Self {
            default_type: ElectricCapacitanceType::MilliFarad,
            v,
        }
    }

    pub fn from_micro_farad(v: f64) -> Self {
        Self {
            default_type: ElectricCapacitanceType::MicroFarad,
            v,
        }
    }

    pub fn from_nano_farad(v: f64) -> Self {
        Self {
            default_type: ElectricCapacitanceType::NanoFarad,
            v,
        }
    }

    pub fn from_pico_farad(v: f64) -> Self {
        Self {
            default_type: ElectricCapacitanceType::PicoFarad,
            v,
        }
    }

    pub fn as_farad(&self) -> f64 {
        match self.default_type {
            ElectricCapacitanceType::Farad => self.v,
            ElectricCapacitanceType::MilliFarad => self.v * 1e-3,
            ElectricCapacitanceType::MicroFarad => self.v * 1e-6,
            ElectricCapacitanceType::NanoFarad => self.v * 1e-9,
            ElectricCapacitanceType::PicoFarad => self.v * 1e-12,
        }
    }

    pub fn as_milli_farad(&self) -> f64 {
        match self.default_type {
            ElectricCapacitanceType::Farad => self.v * 1e3,
            ElectricCapacitanceType::MilliFarad => self.v,
            ElectricCapacitanceType::MicroFarad => self.v * 1e-3,
            ElectricCapacitanceType::NanoFarad => self.v * 1e-6,
            ElectricCapacitanceType::PicoFarad => self.v * 1e-9,
        }
    }

    pub fn as_micro_farad(&self) -> f64 {
        match self.default_type {
            ElectricCapacitanceType::Farad => self.v * 1e6,
            ElectricCapacitanceType::MilliFarad => self.v * 1000.0000000000001,
            ElectricCapacitanceType::MicroFarad => self.v,
            ElectricCapacitanceType::NanoFarad => self.v * 1e-3,
            ElectricCapacitanceType::PicoFarad => self.v * 1e-6,
        }
    }

    pub fn as_nano_farad(&self) -> f64 {
        match self.default_type {
            ElectricCapacitanceType::Farad => self.v * 999999999.9999999,
            ElectricCapacitanceType::MilliFarad => self.v * 1e6,
            ElectricCapacitanceType::MicroFarad => self.v * 999.9999999999999,
            ElectricCapacitanceType::NanoFarad => self.v,
            ElectricCapacitanceType::PicoFarad => self.v * 1e-3,
        }
    }

    pub fn as_pico_farad(&self) -> f64 {
        match self.default_type {
            ElectricCapacitanceType::Farad => self.v * 1000000000000.0,
            ElectricCapacitanceType::MilliFarad => self.v * 1e9,
            ElectricCapacitanceType::MicroFarad => self.v * 1e6,
            ElectricCapacitanceType::NanoFarad => self.v * 1000.0000000000001,
            ElectricCapacitanceType::PicoFarad => self.v,
        }
    }

}

impl Add for ElectricCapacitance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_farad() + rhs.as_farad();
        Self::from_farad(v)
    }
}

impl<'a, 'b> Add<&'b ElectricCapacitance> for &'a ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn add(self, rhs: &'b ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() + rhs.as_farad()) }
}
impl<'a> Add<&'a ElectricCapacitance> for ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn add(self, rhs: &'a ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() + rhs.as_farad()) }
}
impl<'a> Add<ElectricCapacitance> for &'a ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn add(self, rhs: ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() + rhs.as_farad()) }
}

impl Add<f64> for ElectricCapacitance {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricCapacitance { v, default_type: self.default_type }
    }
}

impl Sub for ElectricCapacitance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_farad() - rhs.as_farad();
        Self::from_farad(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricCapacitance> for &'a ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn sub(self, rhs: &'b ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() - rhs.as_farad()) }
}
impl<'a> Sub<&'a ElectricCapacitance> for ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn sub(self, rhs: &'a ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() - rhs.as_farad()) }
}
impl<'a> Sub<ElectricCapacitance> for &'a ElectricCapacitance {
    type Output = ElectricCapacitance;
    fn sub(self, rhs: ElectricCapacitance) -> Self::Output { ElectricCapacitance::from_farad(self.as_farad() - rhs.as_farad()) }
}

impl Sub<f64> for ElectricCapacitance {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricCapacitance { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricCapacitance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_farad() * rhs;
        Self::from_farad(v)
    }
}

impl Div<f64> for ElectricCapacitance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_farad() / rhs;
        Self::from_farad(v)
    }
}

impl Mul<ElectricCapacitance> for f64 {
    type Output = ElectricCapacitance;
    fn mul(self, rhs: ElectricCapacitance) -> Self::Output {
        let v = self * rhs.as_farad();
        ElectricCapacitance::from_farad(v)
    }
}

impl Div<ElectricCapacitance> for f64 {
    type Output = ElectricCapacitance;
    fn div(self, rhs: ElectricCapacitance) -> Self::Output {
        let v = self / rhs.as_farad();
        ElectricCapacitance::from_farad(v)
    }
}

impl Mul<Coef> for ElectricCapacitance {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_farad() * rhs.get_value();
        Self::from_farad(v)
    }
}

impl Div<Coef> for ElectricCapacitance {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_farad() / rhs.get_value();
        Self::from_farad(v)
    }
}

impl Neg for ElectricCapacitance {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_farad();
        Self::from_farad(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_capacitance_create_and_convert() {
        let v1 = ElectricCapacitance::from_farad(1.0);
        assert_relative_eq!(v1.as_farad(), 1.0);
        let v2 = ElectricCapacitance::from_milli_farad(1e3);
        assert_relative_eq!(v2.as_farad(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_milli_farad(), 1e3, epsilon = 1e-8);
        let v2 = ElectricCapacitance::from_micro_farad(1e6);
        assert_relative_eq!(v2.as_farad(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_micro_farad(), 1e6, epsilon = 1e-8);
        let v2 = ElectricCapacitance::from_nano_farad(999999999.9999999);
        assert_relative_eq!(v2.as_farad(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_nano_farad(), 999999999.9999999, epsilon = 1e-8);
        let v2 = ElectricCapacitance::from_pico_farad(1000000000000.0);
        assert_relative_eq!(v2.as_farad(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_pico_farad(), 1000000000000.0, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_capacitance_add_sub() {
        let a = ElectricCapacitance::from_farad(5.0);
        let b = ElectricCapacitance::from_farad(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_farad(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_farad(), 2.0);
    }

    #[test]
    fn test_electric_capacitance_mul_div_f64() {
        let a = ElectricCapacitance::from_farad(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_farad(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_farad(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_farad(), 18.0);
    }

    #[test]
    fn test_electric_capacitance_coef_ops() {
        let a = ElectricCapacitance::from_farad(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_farad(), 20.0);
        assert_relative_eq!((a / c).as_farad(), 5.0);
    }

    #[test]
    fn test_electric_capacitance_neg() {
        let a = ElectricCapacitance::from_farad(5.0);
        let n = -a;
        assert_relative_eq!(n.as_farad(), -5.0);
    }

    #[test]
    fn test_electric_capacitance_default_is_zero() {
        let a = ElectricCapacitance::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_capacitance_ref_ops() {
        let a = ElectricCapacitance::from_farad(2.0);
        let b = ElectricCapacitance::from_farad(3.0);
        assert_relative_eq!((&a + &b).as_farad(), 5.0);
        assert_relative_eq!((&a - &b).as_farad(), -1.0);
        assert_relative_eq!((a + &b).as_farad(), 5.0);
        assert_relative_eq!((&a + b).as_farad(), 5.0);
    }

    #[test]
    fn test_electric_capacitance_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricCapacitance::from_farad(1.23);
        let d = g.as_any().downcast_ref::<ElectricCapacitance>().unwrap();
        assert_relative_eq!(d.as_farad(), 1.23);
    }

    #[test]
    fn test_electric_capacitance_set_value() {
        let mut a = ElectricCapacitance::from_farad(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_farad(), 42.0);
    }
}
