use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricPotential, ElectricPotentialType, PhysicalQuantity, ElectricCurrent, ElectricResistance};
use approx::assert_relative_eq;

impl Default for ElectricPotential {
    fn default() -> Self {
        Self::from_v(0.0)
    }
}

impl PhysicalQuantity for ElectricPotential {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_v()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricPotential (V)".to_string()
    }
}

impl ElectricPotential {
    pub fn from_v(v: f64) -> Self {
        Self {
            default_type: ElectricPotentialType::V,
            v,
        }
    }

    pub fn from_milli_v(v: f64) -> Self {
        Self {
            default_type: ElectricPotentialType::MilliV,
            v,
        }
    }

    pub fn from_micro_v(v: f64) -> Self {
        Self {
            default_type: ElectricPotentialType::MicroV,
            v,
        }
    }

    pub fn from_kilo_v(v: f64) -> Self {
        Self {
            default_type: ElectricPotentialType::KiloV,
            v,
        }
    }

    pub fn as_v(&self) -> f64 {
        match self.default_type {
            ElectricPotentialType::V => self.v,
            ElectricPotentialType::MilliV => self.v * 1e-3,
            ElectricPotentialType::MicroV => self.v * 1e-6,
            ElectricPotentialType::KiloV => self.v * 1e3,
        }
    }

    pub fn as_milli_v(&self) -> f64 {
        match self.default_type {
            ElectricPotentialType::V => self.v * 1e3,
            ElectricPotentialType::MilliV => self.v,
            ElectricPotentialType::MicroV => self.v * 1e-3,
            ElectricPotentialType::KiloV => self.v * 1e6,
        }
    }

    pub fn as_micro_v(&self) -> f64 {
        match self.default_type {
            ElectricPotentialType::V => self.v * 1e6,
            ElectricPotentialType::MilliV => self.v * 1000.0000000000001,
            ElectricPotentialType::MicroV => self.v,
            ElectricPotentialType::KiloV => self.v * 1e9,
        }
    }

    pub fn as_kilo_v(&self) -> f64 {
        match self.default_type {
            ElectricPotentialType::V => self.v * 1e-3,
            ElectricPotentialType::MilliV => self.v * 1e-6,
            ElectricPotentialType::MicroV => self.v * 9.999999999999999e-10,
            ElectricPotentialType::KiloV => self.v,
        }
    }

}

impl Add for ElectricPotential {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_v() + rhs.as_v();
        Self::from_v(v)
    }
}

impl<'a, 'b> Add<&'b ElectricPotential> for &'a ElectricPotential {
    type Output = ElectricPotential;
    fn add(self, rhs: &'b ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() + rhs.as_v()) }
}
impl<'a> Add<&'a ElectricPotential> for ElectricPotential {
    type Output = ElectricPotential;
    fn add(self, rhs: &'a ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() + rhs.as_v()) }
}
impl<'a> Add<ElectricPotential> for &'a ElectricPotential {
    type Output = ElectricPotential;
    fn add(self, rhs: ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() + rhs.as_v()) }
}

impl Add<f64> for ElectricPotential {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricPotential { v, default_type: self.default_type }
    }
}

impl Sub for ElectricPotential {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_v() - rhs.as_v();
        Self::from_v(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricPotential> for &'a ElectricPotential {
    type Output = ElectricPotential;
    fn sub(self, rhs: &'b ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() - rhs.as_v()) }
}
impl<'a> Sub<&'a ElectricPotential> for ElectricPotential {
    type Output = ElectricPotential;
    fn sub(self, rhs: &'a ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() - rhs.as_v()) }
}
impl<'a> Sub<ElectricPotential> for &'a ElectricPotential {
    type Output = ElectricPotential;
    fn sub(self, rhs: ElectricPotential) -> Self::Output { ElectricPotential::from_v(self.as_v() - rhs.as_v()) }
}

impl Sub<f64> for ElectricPotential {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricPotential { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricPotential {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_v() * rhs;
        Self::from_v(v)
    }
}

impl Div<f64> for ElectricPotential {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_v() / rhs;
        Self::from_v(v)
    }
}

impl Mul<ElectricPotential> for f64 {
    type Output = ElectricPotential;
    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        let v = self * rhs.as_v();
        ElectricPotential::from_v(v)
    }
}

impl Div<ElectricPotential> for f64 {
    type Output = ElectricPotential;
    fn div(self, rhs: ElectricPotential) -> Self::Output {
        let v = self / rhs.as_v();
        ElectricPotential::from_v(v)
    }
}

impl Mul<Coef> for ElectricPotential {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_v() * rhs.get_value();
        Self::from_v(v)
    }
}

impl Div<Coef> for ElectricPotential {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_v() / rhs.get_value();
        Self::from_v(v)
    }
}

impl Neg for ElectricPotential {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_v();
        Self::from_v(v)
    }
}


// ElectricCurrent × ElectricResistance → ElectricPotential
impl Mul<ElectricResistance> for ElectricCurrent {
    type Output = ElectricPotential;
    fn mul(self, rhs: ElectricResistance) -> Self::Output {
        ElectricPotential::from_v(self.as_a() * rhs.as_ohm())
    }
}

impl<'a, 'b> Mul<&'b ElectricResistance> for &'a ElectricCurrent {
    type Output = ElectricPotential;
    fn mul(self, rhs: &'b ElectricResistance) -> Self::Output { ElectricPotential::from_v(self.as_a() * rhs.as_ohm()) }
}
impl<'a> Mul<&'a ElectricResistance> for ElectricCurrent {
    type Output = ElectricPotential;
    fn mul(self, rhs: &'a ElectricResistance) -> Self::Output { ElectricPotential::from_v(self.as_a() * rhs.as_ohm()) }
}
impl<'a> Mul<ElectricResistance> for &'a ElectricCurrent {
    type Output = ElectricPotential;
    fn mul(self, rhs: ElectricResistance) -> Self::Output { ElectricPotential::from_v(self.as_a() * rhs.as_ohm()) }
}

// ElectricResistance × ElectricCurrent → ElectricPotential
impl Mul<ElectricCurrent> for ElectricResistance {
    type Output = ElectricPotential;
    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        ElectricPotential::from_v(self.as_ohm() * rhs.as_a())
    }
}

impl<'a, 'b> Mul<&'b ElectricCurrent> for &'a ElectricResistance {
    type Output = ElectricPotential;
    fn mul(self, rhs: &'b ElectricCurrent) -> Self::Output { ElectricPotential::from_v(self.as_ohm() * rhs.as_a()) }
}
impl<'a> Mul<&'a ElectricCurrent> for ElectricResistance {
    type Output = ElectricPotential;
    fn mul(self, rhs: &'a ElectricCurrent) -> Self::Output { ElectricPotential::from_v(self.as_ohm() * rhs.as_a()) }
}
impl<'a> Mul<ElectricCurrent> for &'a ElectricResistance {
    type Output = ElectricPotential;
    fn mul(self, rhs: ElectricCurrent) -> Self::Output { ElectricPotential::from_v(self.as_ohm() * rhs.as_a()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_potential_create_and_convert() {
        let v1 = ElectricPotential::from_v(1.0);
        assert_relative_eq!(v1.as_v(), 1.0);
        let v2 = ElectricPotential::from_milli_v(1e3);
        assert_relative_eq!(v2.as_v(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_milli_v(), 1e3, epsilon = 1e-8);
        let v2 = ElectricPotential::from_micro_v(1e6);
        assert_relative_eq!(v2.as_v(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_micro_v(), 1e6, epsilon = 1e-8);
        let v2 = ElectricPotential::from_kilo_v(1e-3);
        assert_relative_eq!(v2.as_v(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_kilo_v(), 1e-3, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_potential_add_sub() {
        let a = ElectricPotential::from_v(5.0);
        let b = ElectricPotential::from_v(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_v(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_v(), 2.0);
    }

    #[test]
    fn test_electric_potential_mul_div_f64() {
        let a = ElectricPotential::from_v(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_v(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_v(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_v(), 18.0);
    }

    #[test]
    fn test_electric_potential_coef_ops() {
        let a = ElectricPotential::from_v(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_v(), 20.0);
        assert_relative_eq!((a / c).as_v(), 5.0);
    }

    #[test]
    fn test_electric_potential_neg() {
        let a = ElectricPotential::from_v(5.0);
        let n = -a;
        assert_relative_eq!(n.as_v(), -5.0);
    }

    #[test]
    fn test_electric_potential_default_is_zero() {
        let a = ElectricPotential::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_potential_ref_ops() {
        let a = ElectricPotential::from_v(2.0);
        let b = ElectricPotential::from_v(3.0);
        assert_relative_eq!((&a + &b).as_v(), 5.0);
        assert_relative_eq!((&a - &b).as_v(), -1.0);
        assert_relative_eq!((a + &b).as_v(), 5.0);
        assert_relative_eq!((&a + b).as_v(), 5.0);
    }

    #[test]
    fn test_electric_potential_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricPotential::from_v(1.23);
        let d = g.as_any().downcast_ref::<ElectricPotential>().unwrap();
        assert_relative_eq!(d.as_v(), 1.23);
    }

    #[test]
    fn test_electric_potential_set_value() {
        let mut a = ElectricPotential::from_v(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_v(), 42.0);
    }
    #[test]
    fn test_electriccurrent_mul_electricresistance() {
        let l = ElectricCurrent::from_a(2.0);
        let r = ElectricResistance::from_ohm(3.0);
        let result: ElectricPotential = l * r;
        assert_relative_eq!(result.as_v(), 6.0);

        let l2 = ElectricCurrent::from_a(2.0);
        let r2 = ElectricResistance::from_ohm(3.0);
        let result2: ElectricPotential = r2 * l2;
        assert_relative_eq!(result2.as_v(), 6.0);
    }
}
