use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{AngularMomentum, AngularMomentumType, Coef, Distance, Mass, MassType, Momentum, MomentumType, PhysicalQuantity};

impl Default for Momentum {
    fn default() -> Self {
        Self::from_kg_m_s(0.0)
    }
}

impl Momentum {
    pub fn from_kg_m_s(kg_m_s: f64) -> Self {
        Self {
            default_type: MomentumType::KgMperSecond,
            v: kg_m_s,
        }
    }

    pub fn from_kg_km_s(kg_km_s: f64) -> Self {
        Self {
            default_type: MomentumType::KgKmperSecond,
            v: kg_km_s,
        }
    }

    pub fn as_kg_m_s(&self) -> f64 {
        match self.default_type {
            MomentumType::KgMperSecond => self.v,
            MomentumType::KgKmperSecond => self.v * 1000.0,
        }
    }

    pub fn as_kg_km_s(&self) -> f64 {
        match self.default_type {
            MomentumType::KgMperSecond => self.v / 1000.0,
            MomentumType::KgKmperSecond => self.v,
        }
    }
}

impl PhysicalQuantity for Momentum {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg_m_s()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}



impl Add for Momentum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_kg_m_s() + rhs.as_kg_m_s();
        Self::from_kg_m_s(v)
    }
}

impl Add<f64> for Momentum {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m_s() + rhs;
        Self::from_kg_m_s(v)
    }
}

impl Sub for Momentum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_kg_m_s() - rhs.as_kg_m_s();
        Self::from_kg_m_s(v)
    }
}

impl Sub<f64> for Momentum {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m_s() - rhs;
        Self::from_kg_m_s(v)
    }
}

impl Mul<f64> for Momentum {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m_s() * rhs;
        Self::from_kg_m_s(v)
    }
}

impl Div<f64> for Momentum {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_kg_m_s() / rhs;
        Self::from_kg_m_s(v)
    }
}

impl Mul<Momentum> for f64 {
    type Output = Momentum;
    fn mul(self, rhs: Momentum) -> Self::Output {
        rhs * self
    }
}

impl Div<Momentum> for f64 {
    type Output = Momentum;
    fn div(self, rhs: Momentum) -> Self::Output {
        let v = self / rhs.as_kg_m_s();
        Momentum::from_kg_m_s(v)
    }
}

impl Mul<Coef> for Momentum {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg_m_s() * rhs.get_value();
        Self::from_kg_m_s(v)
    }
}

impl Div<Coef> for Momentum {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg_m_s() / rhs.get_value();
        Self::from_kg_m_s(v)
    }
}


impl Mul<Distance> for Momentum {
    type Output = AngularMomentum;

    fn mul(self, rhs: Distance) -> Self::Output {
        let v = self.as_kg_m_s() * rhs.as_m();
        AngularMomentum::from_kg_m2_per_second(v)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_momentum() {
        let m1 = Momentum::from_kg_m_s(1000.0);
        assert_eq!(m1.v, 1000.0);
        assert_eq!(m1.default_type, MomentumType::KgMperSecond);

        let m2 = Momentum::default();
        assert_eq!(m2.v, 0.0);
        assert_eq!(m2.default_type, MomentumType::KgMperSecond);

        let m3 = Momentum::from_kg_km_s(1000.0);
        assert_eq!(m3.v, 1000.0);
        assert_eq!(m3.default_type, MomentumType::KgKmperSecond);
    }

    #[test]
    fn test_momentum_as() {
        let m1 = Momentum::from_kg_m_s(1000.0);
        assert_eq!(m1.as_kg_m_s(), 1000.0);
        assert_relative_eq!(m1.as_kg_km_s(), 1.0);

        let m2 = Momentum::from_kg_km_s(1.0);
        assert_eq!(m2.as_kg_km_s(), 1.0);
        assert_relative_eq!(m2.as_kg_m_s(), 1000.0);
    }

    #[test]
    fn test_momentum_add() {
        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = Momentum::from_kg_m_s(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg_m_s(), 2000.0);

        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = Momentum::from_kg_m_s(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg_m_s(), 2.0 * 1000.0);


        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = m1 + 100.0;
        assert_eq!(m2.as_kg_m_s(), 1100.0);
    }

    #[test]
    fn test_momentum_sub() {
        let m1 = Momentum::from_kg_m_s(2000.0);
        let m2 = Momentum::from_kg_m_s(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m_s(), 1000.0);

        let m1 = Momentum::from_kg_m_s(3000.0);
        let m2 = Momentum::from_kg_m_s(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m_s(), 2.0 * 1000.0);

        let m1 = Momentum::from_kg_m_s(1100.0);
        let m2 = Momentum::from_kg_m_s(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg_m_s(), 100.0);

        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = m1 - 100.0;
        assert_eq!(m2.as_kg_m_s(), 900.0);
    }

    #[test]
    fn test_momentum_mul() {
        let m1 = Momentum::from_kg_m_s(2000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_kg_m_s(), 4000.0);
        let m1 = Momentum::from_kg_m_s(3000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_kg_m_s(), 6000.0);

        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = m1 * Coef::new(2.0);
        assert_eq!(m2.as_kg_m_s(), 2000.0);
    }

    #[test]
    fn test_momentum_div() {
        let m1 = Momentum::from_kg_m_s(2000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_kg_m_s(), 1000.0);
        let m1 = Momentum::from_kg_m_s(3000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_kg_m_s(), 1500.0);
        let m1 = Momentum::from_kg_m_s(1000.0);
        let m2 = m1 / Coef::new(2.0);
        assert_eq!(m2.as_kg_m_s(), 500.0);
    }

    #[test]
    fn test_is_zero() {
        let m1 = Momentum::from_kg_m_s(0.0);
        assert!(m1.is_zero());
        let m1 = Momentum::from_kg_m_s(1.0);
        assert!(!m1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let m1 = Momentum::from_kg_m_s(1.0);
        assert_eq!(m1.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut m1 = Momentum::from_kg_m_s(1.0);
        m1.set_value(1000.0);
        assert_eq!(m1.as_kg_m_s(), 1000.0);
    }

    #[test]
    fn test_momentum_mul_distance() {
        let m1 = Momentum::from_kg_m_s(1000.0);
        let d1 = Distance::from_m(1000.0);
        let m2 = m1 * d1;
        assert_eq!(m2.as_kg_m2_per_second(), 1000.0 * 1000.0);
    }

    #[test]
    fn test_f64_mul_momentum() {
        // f64 * Momentum 测试
        let m = Momentum::from_kg_m_s(2.0);
        let result = 3.0 * m;
        assert_relative_eq!(result.as_kg_m_s(), 6.0);

        let m = Momentum::from_kg_km_s(1.0);
        let result = 2.0 * m;
        assert_relative_eq!(result.as_kg_km_s(), 2.0);
    }

    #[test]
    fn test_f64_div_momentum() {
        // f64 / Momentum 测试
        let m = Momentum::from_kg_m_s(2.0);
        let result = 6.0 / m;
        assert_relative_eq!(result.as_kg_m_s(), 3.0);

        let m = Momentum::from_kg_km_s(1.0);
        let result = 2.0 / m;
        assert_relative_eq!(result.as_kg_km_s(), 2.0);
    }
}