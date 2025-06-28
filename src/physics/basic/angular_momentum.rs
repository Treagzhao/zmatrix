use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{ AngularMomentum, AngularMomentumType, Coef, PhysicalQuantity};

impl Default for AngularMomentum {
    fn default() -> Self {
        Self::from_kg_m2_per_second(0.0)
    }
}


impl PhysicalQuantity for AngularMomentum {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_m2_per_second()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl AngularMomentum {
    pub fn from_kg_m2_per_second(kg: f64) -> Self {
        Self {
            default_type: AngularMomentumType::KgM2perSecond,
            v: kg,
        }
    }

    pub fn from_kg_km2_per_second(kg: f64) -> Self {
        Self {
            default_type: AngularMomentumType::KgKm2perSecond,
            v: kg,
        }
    }

    pub fn as_kg_m2_per_second(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v,
            AngularMomentumType::KgKm2perSecond => self.v * 1e6,
        }
    }

    pub fn as_kg_km2_per_second(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v * 1e-6,
            AngularMomentumType::KgKm2perSecond => self.v,
        }
    }
}


impl Add for AngularMomentum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_kg_m2_per_second() + rhs.as_kg_m2_per_second();
        Self::from_kg_m2_per_second(v)
    }
}

impl Add<f64> for AngularMomentum {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m2_per_second() + rhs;
        Self::from_kg_m2_per_second(v)
    }
}

impl Neg for AngularMomentum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_kg_m2_per_second();
        Self::from_kg_m2_per_second(v)
    }
}

impl Sub for AngularMomentum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_kg_m2_per_second() - rhs.as_kg_m2_per_second();
        Self::from_kg_m2_per_second(v)
    }
}

impl Sub<f64> for AngularMomentum {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m2_per_second() - rhs;
        Self::from_kg_m2_per_second(v)
    }
}

impl Mul<f64> for AngularMomentum {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m2_per_second() * rhs;
        Self::from_kg_m2_per_second(v)
    }
}

impl Div<f64> for AngularMomentum {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m2_per_second() / rhs;
        Self::from_kg_m2_per_second(v)
    }
}

impl Mul<Coef> for AngularMomentum {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg_m2_per_second() * rhs.get_value();
        Self::from_kg_m2_per_second(v)
    }
}

impl Div<Coef> for AngularMomentum {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg_m2_per_second() / rhs.get_value();
        Self::from_kg_m2_per_second(v)
    }
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_angular_momentum() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        assert_eq!(m1.v, 1000.0);
        assert_eq!(m1.default_type, AngularMomentumType::KgM2perSecond);

        let m2 = AngularMomentum::default();
        assert_eq!(m2.v, 0.0);
        assert_eq!(m2.default_type, AngularMomentumType::KgM2perSecond);

        let m3 = AngularMomentum::from_kg_km2_per_second(1000.0);
        assert_eq!(m3.v, 1000.0);
        assert_eq!(m3.default_type, AngularMomentumType::KgKm2perSecond);
    }

    #[test]
    fn test_angular_momentum_as() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        assert_eq!(m1.as_kg_m2_per_second(), 1000.0);
        assert_relative_eq!(m1.as_kg_km2_per_second(), 1e-3);

        let m2 = AngularMomentum::from_kg_km2_per_second(1000.0);
        assert_eq!(m2.as_kg_m2_per_second(), 1e9);
        assert_relative_eq!(m2.as_kg_km2_per_second(), 1000.0);
    }

    #[test]
    fn test_angular_momentum_add() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg_m2_per_second(), 2000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg_m2_per_second(), 2.0 * 1000.0);


        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 + 100.0;
        assert_eq!(m2.as_kg_m2_per_second(), 1100.0);
    }

    #[test]
    fn test_angular_momentum_sub() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m2_per_second(), 1000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m2_per_second(), 2.0 * 1000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1100.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m2_per_second(), 100.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 - 100.0;
        assert_eq!(m2.as_kg_m2_per_second(), 900.0);
    }

    #[test]
    fn test_angular_momentum_mul() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_kg_m2_per_second(), 4000.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_kg_m2_per_second(), 6000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 * Coef::new(2.0);
        assert_eq!(m2.as_kg_m2_per_second(), 2000.0);
    }

    #[test]
    fn test_angular_momentum_div() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_kg_m2_per_second(), 1000.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_kg_m2_per_second(), 1500.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 / Coef::new(2.0);
        assert_eq!(m2.as_kg_m2_per_second(), 500.0);
    }

    #[test]
    fn test_is_zero() {
        let m1 = AngularMomentum::from_kg_m2_per_second(0.0);
        assert!(m1.is_zero());
        let m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        assert!(!m1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        assert_eq!(m1.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        m1.set_value(1000.0);
        assert_eq!(m1.as_kg_m2_per_second(), 1000.0);
    }

    #[test]
    fn test_negative() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = -m1;
        assert_eq!(m2.as_kg_m2_per_second(), -1000.0);
    }
    #[test]
    fn test_div() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(500.0);
        let m3 = m1 / m2;
        assert_eq!(m3.get_value(), 2.0);
    }
}