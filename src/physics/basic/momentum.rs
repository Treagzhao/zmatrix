use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
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

// 引用-引用 与 混合引用：Momentum 加法
impl<'a, 'b> Add<&'b Momentum> for &'a Momentum {
    type Output = Momentum;
    fn add(self, rhs: &'b Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() + rhs.as_kg_m_s()) }
}
impl<'a> Add<&'a Momentum> for Momentum {
    type Output = Momentum;
    fn add(self, rhs: &'a Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() + rhs.as_kg_m_s()) }
}
impl<'a> Add<Momentum> for &'a Momentum {
    type Output = Momentum;
    fn add(self, rhs: Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() + rhs.as_kg_m_s()) }
}

impl Add<f64> for Momentum {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Momentum {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for Momentum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_kg_m_s() - rhs.as_kg_m_s();
        Self::from_kg_m_s(v)
    }
}

// 引用-引用 与 混合引用：Momentum 减法
impl<'a, 'b> Sub<&'b Momentum> for &'a Momentum {
    type Output = Momentum;
    fn sub(self, rhs: &'b Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() - rhs.as_kg_m_s()) }
}
impl<'a> Sub<&'a Momentum> for Momentum {
    type Output = Momentum;
    fn sub(self, rhs: &'a Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() - rhs.as_kg_m_s()) }
}
impl<'a> Sub<Momentum> for &'a Momentum {
    type Output = Momentum;
    fn sub(self, rhs: Momentum) -> Self::Output { Momentum::from_kg_m_s(self.as_kg_m_s() - rhs.as_kg_m_s()) }
}

impl Sub<f64> for Momentum {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Momentum {
            v,
            default_type: self.default_type,
        }
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
        let v = self / rhs.v;
        Momentum {
            default_type: rhs.default_type,
            v: v,
        }
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

impl Neg for Momentum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_kg_m_s();
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

// 引用版本：Momentum * Distance -> AngularMomentum
impl<'a, 'b> Mul<&'b Distance> for &'a Momentum {
    type Output = AngularMomentum;
    fn mul(self, rhs: &'b Distance) -> Self::Output { AngularMomentum::from_kg_m2_per_second(self.as_kg_m_s() * rhs.as_m()) }
}
impl<'a> Mul<&'a Distance> for Momentum {
    type Output = AngularMomentum;
    fn mul(self, rhs: &'a Distance) -> Self::Output { AngularMomentum::from_kg_m2_per_second(self.as_kg_m_s() * rhs.as_m()) }
}
impl<'a> Mul<Distance> for &'a Momentum {
    type Output = AngularMomentum;
    fn mul(self, rhs: Distance) -> Self::Output { AngularMomentum::from_kg_m2_per_second(self.as_kg_m_s() * rhs.as_m()) }
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

    #[test]
    fn test_momentum_ref_ops() {
        let m1 = Momentum::from_kg_m_s(2.0);
        let m2 = Momentum::from_kg_km_s(0.001); // 1 m/s * kg
        let s = &m1 + &m2;
        assert_relative_eq!(s.as_kg_m_s(), 3.0);

        let d = &m1 - &m2;
        assert_relative_eq!(d.as_kg_m_s(), 1.0);

        let l = &m1 * &Distance::from_m(3.0);
        assert_relative_eq!(l.as_kg_m2_per_second(), 6.0);

        // 混合引用：加/减
        let s2 = m1 + &m2;
        assert_relative_eq!(s2.as_kg_m_s(), 3.0);
        let m1b = Momentum::from_kg_m_s(2.0);
        let s3 = &m1b + m2;
        assert_relative_eq!(s3.as_kg_m_s(), 3.0);

        let m1c = Momentum::from_kg_m_s(2.0);
        let m2c = Momentum::from_kg_km_s(0.001);
        let d2 = m1c - &m2c;
        assert_relative_eq!(d2.as_kg_m_s(), 1.0);
        let m1d = Momentum::from_kg_m_s(2.0);
        let m2d = Momentum::from_kg_km_s(0.001);
        let d3 = &m1d - m2d;
        assert_relative_eq!(d3.as_kg_m_s(), 1.0);

        // 混合引用：* Distance
        let m1e = Momentum::from_kg_m_s(2.0);
        let l2 = m1e * &Distance::from_m(3.0);
        assert_relative_eq!(l2.as_kg_m2_per_second(), 6.0);
        let m1f = Momentum::from_kg_m_s(2.0);
        let l3 = &m1f * Distance::from_m(3.0);
        assert_relative_eq!(l3.as_kg_m2_per_second(), 6.0);
    }

    #[test]
    fn test_momentum_neg() {
        // 测试正值的负号
        let m1 = Momentum::from_kg_m_s(5.0);
        let neg_m1 = -m1;
        assert_relative_eq!(neg_m1.as_kg_m_s(), -5.0);

        // 测试负值的负号
        let m2 = Momentum::from_kg_km_s(-1.0);
        let neg_m2 = -m2;
        assert_relative_eq!(neg_m2.as_kg_km_s(), 1.0);

        // 测试零值
        let m3 = Momentum::from_kg_m_s(0.0);
        let neg_m3 = -m3;
        assert_relative_eq!(neg_m3.as_kg_m_s(), 0.0);

        // 测试不同单位的负号操作
        let m4 = Momentum::from_kg_km_s(1.0);
        let neg_m4 = -m4;
        assert_relative_eq!(neg_m4.as_kg_m_s(), -1000.0);

        // 测试大数值
        let m5 = Momentum::from_kg_m_s(2.0);
        let neg_m5 = -m5;
        assert_relative_eq!(neg_m5.as_kg_km_s(), -0.002);
    }
}