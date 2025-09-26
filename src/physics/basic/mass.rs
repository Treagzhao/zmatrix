use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, Mass, MassType, Momentum, PhysicalQuantity, Velocity};
use approx::assert_relative_eq;

impl Default for Mass {
    fn default() -> Self {
        Self::from_kg(0.0)
    }
}


impl PhysicalQuantity for Mass {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kg()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Mass {
    pub fn from_kg(kg: f64) -> Self {
        Self {
            default_type: MassType::Kg,
            v: kg,
        }
    }

    pub fn from_g(g: f64) -> Self {
        Self {
            default_type: MassType::g,
            v: g,
        }
    }

    pub fn as_kg(&self) -> f64 {
        match self.default_type {
            MassType::Kg => self.v,
            MassType::g => self.v / 1000.0,
        }
    }

    pub fn as_g(&self) -> f64 {
        match self.default_type {
            MassType::Kg => self.v * 1000.0,
            MassType::g => self.v,
        }
    }
}


impl Add for Mass {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_kg() + rhs.as_kg();
        Self::from_kg(v)
    }
}

// 引用-引用 与 混合引用：Mass 加法
impl<'a, 'b> Add<&'b Mass> for &'a Mass {
    type Output = Mass;
    fn add(self, rhs: &'b Mass) -> Self::Output { Mass::from_kg(self.as_kg() + rhs.as_kg()) }
}
impl<'a> Add<&'a Mass> for Mass {
    type Output = Mass;
    fn add(self, rhs: &'a Mass) -> Self::Output { Mass::from_kg(self.as_kg() + rhs.as_kg()) }
}
impl<'a> Add<Mass> for &'a Mass {
    type Output = Mass;
    fn add(self, rhs: Mass) -> Self::Output { Mass::from_kg(self.as_kg() + rhs.as_kg()) }
}

impl Add<f64> for Mass {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Mass {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for Mass {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_kg() - rhs.as_kg();
        Self::from_kg(v)
    }
}

// 引用-引用 与 混合引用：Mass 减法
impl<'a, 'b> Sub<&'b Mass> for &'a Mass {
    type Output = Mass;
    fn sub(self, rhs: &'b Mass) -> Self::Output { Mass::from_kg(self.as_kg() - rhs.as_kg()) }
}
impl<'a> Sub<&'a Mass> for Mass {
    type Output = Mass;
    fn sub(self, rhs: &'a Mass) -> Self::Output { Mass::from_kg(self.as_kg() - rhs.as_kg()) }
}
impl<'a> Sub<Mass> for &'a Mass {
    type Output = Mass;
    fn sub(self, rhs: Mass) -> Self::Output { Mass::from_kg(self.as_kg() - rhs.as_kg()) }
}

impl Sub<f64> for Mass {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Mass {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Mass {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_kg() * rhs;
        Self::from_kg(v)
    }
}

impl Div<f64> for Mass {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_kg() / rhs;
        Self::from_kg(v)
    }
}

impl Mul<Mass> for f64 {
    type Output = Mass;
    fn mul(self, rhs: Mass) -> Self::Output {
        rhs * self
    }
}

impl Div<Mass> for f64 {
    type Output = Mass;
    fn div(self, rhs: Mass) -> Self::Output {
        let v = self / rhs.v;
        Mass {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for Mass {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg() * rhs.get_value();
        Self::from_kg(v)
    }
}

impl Div<Coef> for Mass {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_kg() / rhs.get_value();
        Self::from_kg(v)
    }
}

impl Neg for Mass {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_kg();
        Self::from_kg(v)
    }
}


impl Mul<Velocity> for Mass {
    type Output = Momentum;
    fn mul(self, rhs: Velocity) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mass() {
        let m1 = Mass::from_kg(1000.0);
        assert_eq!(m1.v, 1000.0);
        assert_eq!(m1.default_type, MassType::Kg);

        let m2 = Mass::from_g(1000.0);
        assert_eq!(m2.v, 1000.0);
        assert_eq!(m2.default_type, MassType::g);

        let m2 = Mass::default();
        assert_eq!(m2.v, 0.0);
        assert_eq!(m2.default_type, MassType::Kg);
    }

    #[test]
    fn test_mass_as() {
        let m1 = Mass::from_kg(1000.0);
        let m2 = m1.as_g();
        assert_eq!(m1.as_g(), 1000.0 * 1000.0);
        assert_eq!(m1.as_kg(), 1000.0);


        let m2 = Mass::from_g(1000.0);
        assert_eq!(m2.as_g(), 1000.0);
        assert_eq!(m2.as_kg(), 1000.0 / 1000.0);
    }

    #[test]
    fn test_mass_add() {
        let m1 = Mass::from_kg(1000.0);
        let m2 = Mass::from_kg(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg(), 2000.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = Mass::from_g(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_g(), 2.0 * 1000.0);

        let m1 = Mass::from_kg(1000.0);
        let m2 = Mass::from_g(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg(), 1001.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = Mass::from_kg(1.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_kg(), 2.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = m1 + 100.0;
        assert_eq!(m2.as_kg(), 1.1);

        let m1 = Mass::from_kg(1200.0);
        let m2 = m1 + 10.2;
        assert_eq!(m2.as_kg(), 1210.2);
    }

    #[test]
    fn test_mass_sub() {
        let m1 = Mass::from_kg(2000.0);
        let m2 = Mass::from_kg(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg(), 1000.0);

        let m1 = Mass::from_g(3000.0);
        let m2 = Mass::from_g(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_g(), 2.0 * 1000.0);

        let m1 = Mass::from_kg(1000.0);
        let m2 = Mass::from_g(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg(), 999.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = Mass::from_kg(1.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_kg(), 0.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = m1 - 100.0;
        assert_eq!(m2.as_kg(), 0.9);

        let m1 = Mass::from_kg(1200.0);
        let m2 = m1 - 10.0;
        assert_eq!(m2.as_kg(), 1190.0);
    }

    #[test]
    fn test_mass_mul() {
        let m1 = Mass::from_kg(2000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_kg(), 4000.0);
        let m1 = Mass::from_g(3000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_g(), 6000.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = m1 * Coef::new(2.0);
        assert_eq!(m2.as_kg(), 2.0);

        let m1 = Mass::from_g(1000.0);
        let m2 = m1 * Coef::new(2.0);
        assert_eq!(m2.as_kg(), 2.0);
    }

    #[test]
    fn test_mass_div() {
        let m1 = Mass::from_kg(2000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_kg(), 1000.0);
        let m1 = Mass::from_g(3000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_g(), 1500.0);
        let m1 = Mass::from_g(1000.0);
        let m2 = m1 / Coef::new(2.0);
        assert_eq!(m2.as_kg(), 0.5);
    }

    #[test]
    fn test_is_zero() {
        let m1 = Mass::from_kg(0.0);
        assert!(m1.is_zero());
        let m1 = Mass::from_kg(1.0);
        assert!(!m1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let m1 = Mass::from_kg(1.0);
        assert_eq!(m1.default_unit_value(), 1.0);
        let m1 = Mass::from_g(1000.0);
        assert_eq!(m1.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut m1 = Mass::from_kg(1.0);
        m1.set_value(1000.0);
        assert_eq!(m1.as_kg(), 1000.0);
    }

    #[test]
    fn test_mass_mul_velocity() {
        let m1 = Mass::from_kg(1000.0);
        let v1 = Velocity::from_m_per_sec(1000.0);
        let m2 = m1 * v1;
        assert_eq!(m2.as_kg_m_s(), 1000.0 * 1000.0);
    }

    #[test]
    fn test_f64_mul_mass() {
        // f64 * Mass 测试
        let m = Mass::from_kg(2.0);
        let result = 3.0 * m;
        assert_relative_eq!(result.as_kg(), 6.0);

        let m = Mass::from_g(1000.0);
        let result = 2.0 * m;
        assert_relative_eq!(result.as_g(), 2000.0);
    }

    #[test]
    fn test_f64_div_mass() {
        // f64 / Mass 测试
        let m = Mass::from_kg(2.0);
        let result = 6.0 / m;
        assert_relative_eq!(result.as_kg(), 3.0);

        let m = Mass::from_g(1000.0);
        let result = 2000.0 / m;
        assert_relative_eq!(result.as_g(), 2.0);
    }

    #[test]
    fn test_mass_ref_ops() {
        let m1 = Mass::from_kg(2.0);
        let m2 = Mass::from_g(1000.0); // 1 kg
        let s = &m1 + &m2;
        assert_relative_eq!(s.as_kg(), 3.0);

        let d = &m1 - &m2;
        assert_relative_eq!(d.as_kg(), 1.0);

        // 混合引用：加/减
        let s2 = m1 + &m2;
        assert_relative_eq!(s2.as_kg(), 3.0);
        let m1b = Mass::from_kg(2.0);
        let s3 = &m1b + m2;
        assert_relative_eq!(s3.as_kg(), 3.0);

        let m1c = Mass::from_kg(2.0);
        let m2c = Mass::from_g(1000.0);
        let d2 = m1c - &m2c;
        assert_relative_eq!(d2.as_kg(), 1.0);
        let m1d = Mass::from_kg(2.0);
        let m2d = Mass::from_g(1000.0);
        let d3 = &m1d - m2d;
        assert_relative_eq!(d3.as_kg(), 1.0);
    }

    #[test]
    fn test_mass_neg() {
        // 测试正值的负号
        let m1 = Mass::from_kg(5.0);
        let neg_m1 = -m1;
        assert_relative_eq!(neg_m1.as_kg(), -5.0);

        // 测试负值的负号
        let m2 = Mass::from_g(-1000.0);
        let neg_m2 = -m2;
        assert_relative_eq!(neg_m2.as_g(), 1000.0);

        // 测试零值
        let m3 = Mass::from_kg(0.0);
        let neg_m3 = -m3;
        assert_relative_eq!(neg_m3.as_kg(), 0.0);

        // 测试不同单位的负号操作
        let m4 = Mass::from_g(1000.0);
        let neg_m4 = -m4;
        assert_relative_eq!(neg_m4.as_kg(), -1.0);

        // 测试大数值
        let m5 = Mass::from_kg(2.0);
        let neg_m5 = -m5;
        assert_relative_eq!(neg_m5.as_g(), -2000.0);
    }
}